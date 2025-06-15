//! United Health MRF source implementation
//! 
//! This module provides functionality to discover and download MRF files
//! from United Health's transparency in coverage website.
//! 
//! # Architecture
//! 
//! The module fetches data in two stages:
//! 1. Fetch the blob list from the API endpoint to get all available index files
//! 2. Fetch each index file in parallel to discover actual MRF file URLs
//! 
//! All operations are designed for maximum concurrency and speed.
//! 
//! # Example
//! 
//! ```no_run
//! use mrf_rs::sources::united_health::UnitedHealthSource;
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let source = UnitedHealthSource::new()?;
//! let files = source.discover_files().await?;
//! println!("Found {} MRF files", files.len());
//! # Ok(())
//! # }
//! ```

use super::{
    base::{BaseSource, utils},
    CompressionType, FetchOptions, MrfFileInfo, MrfFileType, MrfSource, ProgressCallback,
    SourceConfig, SourceError, SourceResult,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures_util::stream::{self, StreamExt, FuturesUnordered};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// United Health specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitedHealthConfig {
    /// Base URL for the transparency website
    pub transparency_url: String,
    /// API endpoint for fetching blob list
    pub api_endpoint: String,
}

impl Default for UnitedHealthConfig {
    fn default() -> Self {
        Self {
            transparency_url: "https://transparency-in-coverage.uhc.com/".to_string(),
            api_endpoint: "https://transparency-in-coverage.uhc.com/api/v1/uhc/blobs".to_string(),
        }
    }
}

/// United Health MRF source
pub struct UnitedHealthSource {
    base: BaseSource,
    config: UnitedHealthConfig,
}

impl UnitedHealthSource {
    /// Create a new United Health source with default configuration
    pub fn new() -> SourceResult<Self> {
        Self::with_config(UnitedHealthConfig::default())
    }
    
    /// Create a new United Health source with custom configuration
    pub fn with_config(config: UnitedHealthConfig) -> SourceResult<Self> {
        let mut source_config = SourceConfig::default();
        source_config.base_url = config.transparency_url.clone();
        source_config.user_agent = Some("mrf-rs/0.1.0 (United Health MRF Fetcher)".to_string());
        
        let base = BaseSource::new(
            "United Health".to_string(),
            "united_health".to_string(),
            source_config,
        )?;
        
        Ok(Self { base, config })
    }
    
    /// Fetch all index files from the API
    pub async fn fetch_all_index_files(&self) -> SourceResult<Vec<IndexFileEntry>> {
        info!("Fetching United Health index files from API");
        
        let response = self.base.http_client.get(&self.config.api_endpoint).await?;
        let content = response.text().await.map_err(SourceError::Http)?;
        
        let api_response: BlobsApiResponse = serde_json::from_str(&content)
            .map_err(|e| SourceError::Parse(format!("Failed to parse blobs API response: {}", e)))?;
        
        // Process blob entries concurrently for better performance
        let entries: Vec<IndexFileEntry> = stream::iter(api_response.blobs)
            .map(|blob| async move {
                IndexFileEntry {
                    name: blob.name.clone(),
                    url: blob.download_url,
                    date: extract_date_from_filename(&blob.name),
                }
            })
            .buffer_unordered(usize::MAX) // No concurrency limit
            .collect()
            .await;
        
        info!("Found {} index files from API", entries.len());
        Ok(entries)
    }
    
    /// Fetch and parse a single index file to get MRF file listings
    async fn fetch_index_file(&self, url: &str) -> SourceResult<Vec<MrfFileInfo>> {
        debug!("Fetching index file: {}", url);
        
        let response = self.base.http_client.get(url).await?;
        
        // Check Content-Length header to skip empty files before downloading
        if let Some(content_length) = response.headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
        {
            if content_length == 0 {
                debug!("Skipping empty index file (Content-Length: 0): {}", url);
                return Ok(Vec::new());
            }
        }
        
        let content = response.bytes().await.map_err(SourceError::Http)?;
        
        // Skip empty files
        if content.is_empty() {
            debug!("Skipping empty index file: {}", url);
            return Ok(Vec::new());
        }
        
        // Check if content is too small to be valid JSON (less than 2 bytes for "{}")
        if content.len() < 2 {
            debug!("Skipping index file with insufficient content ({}B): {}", content.len(), url);
            return Ok(Vec::new());
        }
        
        // Offload deserialization to a blocking thread
        let index: IndexFile = match tokio::task::spawn_blocking({
            let content = content.clone();
            move || serde_json::from_slice(&content)
        }).await.map_err(|e| SourceError::Other(format!("Join error: {}", e)))? {
            Ok(idx) => idx,
            Err(e) => {
                // Log the first few bytes to help debug
                let preview = if content.len() > 100 {
                    format!("{:?}...", &content[..100])
                } else {
                    format!("{:?}", &content[..])
                };
                debug!("Failed to parse index file {}: {}. Content preview: {}", url, e, preview);
                return Ok(Vec::new());
            }
        };
        
        // Use iterators to process all files from all reporting structures
        let files: Vec<MrfFileInfo> = index.reporting_structure
            .into_iter()
            .flat_map(|structure| {
                // Chain in-network and allowed amount files
                let in_network_files = structure.in_network_files
                    .unwrap_or_default()
                    .into_iter()
                    .map(|file| create_mrf_file_info(
                        file,
                        MrfFileType::InNetwork,
                        &index.reporting_entity_name,
                        &index.reporting_entity_type,
                        url,
                    ));
                
                let allowed_amount_files = structure.allowed_amount_files
                    .unwrap_or_default()
                    .into_iter()
                    .map(|file| create_mrf_file_info(
                        file,
                        MrfFileType::AllowedAmount,
                        &index.reporting_entity_name,
                        &index.reporting_entity_type,
                        url,
                    ));
                
                in_network_files.chain(allowed_amount_files)
            })
            .collect();
        
        debug!("Found {} files in index", files.len());
        Ok(files)
    }
    
    /// Fetch multiple MRF files in parallel
    /// 
    /// This method downloads multiple MRF files concurrently with configurable concurrency.
    /// Returns a vector of results, where each result contains either the file data or an error.
    /// 
    /// # Arguments
    /// 
    /// * `files` - Vector of MRF file information to download
    /// * `options` - Optional fetch options to apply to all downloads
    /// * `max_concurrent_downloads` - Maximum number of concurrent downloads (defaults to no limit)
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// # use mrf_rs::sources::united_health::UnitedHealthSource;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let source = UnitedHealthSource::new()?;
    /// let files = source.discover_files().await?;
    /// 
    /// // Download first 10 files with max 5 concurrent downloads
    /// let results = source.fetch_all_files(
    ///     files.into_iter().take(10).collect(),
    ///     None,
    ///     Some(5)
    /// ).await;
    /// 
    /// for (file_info, result) in results {
    ///     match result {
    ///         Ok(data) => println!("Downloaded {}: {} bytes", file_info.name, data.len()),
    ///         Err(e) => eprintln!("Failed to download {}: {}", file_info.name, e),
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_all_files(
        &self,
        files: Vec<MrfFileInfo>,
        options: Option<FetchOptions>,
        max_concurrent_downloads: Option<usize>,
    ) -> Vec<(MrfFileInfo, SourceResult<Vec<u8>>)> {
        let max_concurrency = max_concurrent_downloads.unwrap_or(usize::MAX);
        let options = options.unwrap_or_default();
        
        info!("Fetching {} MRF files with max concurrency of {}", 
              files.len(), if max_concurrency == usize::MAX { "unlimited".to_string() } else { max_concurrency.to_string() });
        
        // Create shared references for async closures
        let self_arc = Arc::new(self);
        let options_arc = Arc::new(options);
        
        // Process all files in parallel with maximum concurrency
        let results: Vec<(MrfFileInfo, SourceResult<Vec<u8>>)> = stream::iter(files)
            .map(|file_info| {
                let self_clone = Arc::clone(&self_arc);
                let options_clone = Arc::clone(&options_arc);
                let file_info_clone = file_info.clone();
                
                async move {
                    let result = self_clone.fetch_file(&file_info_clone, Some((*options_clone).clone())).await;
                    (file_info, result)
                }
            })
            .buffer_unordered(max_concurrency)
            .collect()
            .await;
        
        let successful = results.iter().filter(|(_, r)| r.is_ok()).count();
        let failed = results.len() - successful;
        
        info!("Fetch complete: {} successful, {} failed", successful, failed);
        
        results
    }
    
    /// Fetch multiple MRF files to disk in parallel
    /// 
    /// This method downloads multiple MRF files concurrently and saves them to a directory.
    /// Each file is saved with a filename based on its ID and original extension.
    /// 
    /// # Arguments
    /// 
    /// * `files` - Vector of MRF file information to download
    /// * `output_dir` - Directory to save files to
    /// * `options` - Optional fetch options to apply to all downloads
    /// * `max_concurrent_downloads` - Maximum number of concurrent downloads
    /// * `progress` - Optional progress callback that receives (completed_files, total_files)
    /// 
    /// # Returns
    /// 
    /// Vector of tuples containing the file info and the result (either the saved path or error)
    pub async fn fetch_all_files_to_disk(
        &self,
        files: Vec<MrfFileInfo>,
        output_dir: &Path,
        options: Option<FetchOptions>,
        max_concurrent_downloads: Option<usize>,
        progress: Option<Box<dyn Fn(usize, usize) + Send + Sync>>,
    ) -> Vec<(MrfFileInfo, SourceResult<std::path::PathBuf>)> {
        use std::sync::atomic::{AtomicUsize, Ordering};
        
        let max_concurrency = max_concurrent_downloads.unwrap_or(usize::MAX);
        let total_files = files.len();
        
        // Create output directory if it doesn't exist
        if let Err(e) = tokio::fs::create_dir_all(output_dir).await {
            let error_msg = format!("Failed to create output directory: {}", e);
            return files.into_iter()
                .map(|f| (f, Err(SourceError::Other(error_msg.clone()))))
                .collect();
        }
        
        info!("Fetching {} MRF files to {} with max concurrency of {}", 
              total_files, output_dir.display(), if max_concurrency == usize::MAX { "unlimited".to_string() } else { max_concurrency.to_string() });
        
        // Create shared references
        let self_arc = Arc::new(self);
        let options_arc = Arc::new(options.unwrap_or_default());
        let progress_arc = Arc::new(progress);
        let completed_count = Arc::new(AtomicUsize::new(0));
        
        // Process all files in parallel
        let results: Vec<(MrfFileInfo, SourceResult<std::path::PathBuf>)> = stream::iter(files)
            .map(|file_info| {
                let self_clone = Arc::clone(&self_arc);
                let options_clone = Arc::clone(&options_arc);
                let progress_clone = Arc::clone(&progress_arc);
                let completed_clone = Arc::clone(&completed_count);
                let file_info_clone = file_info.clone();
                let output_dir = output_dir.to_path_buf();
                
                async move {
                    // Generate filename from file ID and URL extension
                    let extension = file_info_clone.url
                        .split('/')
                        .last()
                        .and_then(|name| name.split('.').last())
                        .unwrap_or("json");
                    
                    let filename = format!("{}_{}.{}", 
                        file_info_clone.file_type.as_str(),
                        file_info_clone.id,
                        extension
                    );
                    let file_path = output_dir.join(filename);
                    
                    // Download file
                    let result = self_clone
                        .fetch_file_to_path(
                            &file_info_clone,
                            &file_path,
                            Some((*options_clone).clone()),
                            None
                        )
                        .await
                        .map(|_| file_path);
                    
                    // Update progress
                    let completed = completed_clone.fetch_add(1, Ordering::SeqCst) + 1;
                    if let Some(ref callback) = *progress_clone {
                        callback(completed, total_files);
                    }
                    
                    (file_info, result)
                }
            })
            .buffer_unordered(max_concurrency)
            .collect()
            .await;
        
        let successful = results.iter().filter(|(_, r)| r.is_ok()).count();
        let failed = results.len() - successful;
        
        info!("Download complete: {} successful, {} failed", successful, failed);
        
        results
    }
}

#[async_trait]
impl MrfSource for UnitedHealthSource {
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn source_id(&self) -> &str {
        &self.base.source_id
    }
    
    async fn discover_files(&self) -> SourceResult<Vec<MrfFileInfo>> {
        // Fetch all index files
        let index_entries = self.fetch_all_index_files().await?;
        
        if index_entries.is_empty() {
            warn!("No index files found");
            return Ok(Vec::new());
        }
        
        let index_count = index_entries.len();
        info!("Processing {} index files with unlimited concurrency", index_count);
        
        // Create shared reference for async closures
        let self_arc = Arc::new(self);
        
        // Process all index files in parallel with maximum concurrency
        let start_time = std::time::Instant::now();
        
        // Create all futures immediately to start them in parallel        
        let all_files: Vec<MrfFileInfo> = stream::iter(index_entries.into_iter().enumerate())
            .map(|(idx, entry)| {
                let self_clone = Arc::clone(&self_arc);
                let start_time_clone = start_time.clone();
                async move {
                    let task_start = std::time::Instant::now();
                    let time_since_start = start_time_clone.elapsed();
                    info!("Starting fetch for index {} at {:.2?}: {}", idx + 1, time_since_start, entry.name);
                    
                    // Use the fetch_index_file method
                    let files = match self_clone.fetch_index_file(&entry.url).await {
                        Ok(files) => files,
                        Err(e) => {
                            debug!("Failed to fetch index file {}: {}", entry.name, e);
                            Vec::new()
                        }
                    };
                    
                    let task_duration = task_start.elapsed();
                    let total_elapsed = start_time_clone.elapsed();
                    if !files.is_empty() {
                        info!("Completed index {} at {:.2?} (task took {:.2?}): {} - found {} files", 
                             idx + 1, total_elapsed, task_duration, entry.name, files.len());
                    }
                    
                    files
                }
            })
            .buffer_unordered(usize::MAX)
            .collect::<Vec<Vec<MrfFileInfo>>>()
            .await
            .into_iter()
            .flatten()
            .collect();
        
        let total_duration = start_time.elapsed();
        info!("Processed all index files in {:.2?} ({:.2} files/sec)", 
              total_duration, 
              index_count as f64 / total_duration.as_secs_f64());
        
        info!("Total MRF files discovered: {}", all_files.len());
        Ok(all_files)
    }
    
    async fn fetch_file(
        &self,
        file_info: &MrfFileInfo,
        options: Option<FetchOptions>,
    ) -> SourceResult<Vec<u8>> {
        let options = options.unwrap_or_default();
        
        info!("Downloading file: {}", file_info.name);
        let response = self.base.http_client.get(&file_info.url).await?;
        
        // Check file size if max_size is specified
        if let Some(max_size) = options.max_size {
            if let Some(content_length) = response.headers()
                .get("content-length")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
            {
                if content_length > max_size {
                    return Err(SourceError::Other(format!(
                        "File size {} exceeds maximum allowed size {}",
                        content_length, max_size
                    )));
                }
            }
        }
        
        let data = response.bytes().await.map_err(SourceError::Http)?;
        Ok(data.to_vec())
    }
    
    async fn fetch_file_to_path(
        &self,
        file_info: &MrfFileInfo,
        path: &Path,
        _options: Option<FetchOptions>,
        progress: Option<ProgressCallback>,
    ) -> SourceResult<()> {
        // Direct download to path
        self.base.http_client.download_file(&file_info.url, path, progress).await
    }
    
    async fn get_metadata(&self) -> SourceResult<serde_json::Value> {
        let index_entries = self.fetch_all_index_files().await?;
        
        Ok(serde_json::json!({
            "source": self.name(),
            "source_id": self.source_id(),
            "transparency_url": self.config.transparency_url,
            "api_endpoint": self.config.api_endpoint,
            "index_file_count": index_entries.len(),
            "index_files": index_entries.iter().map(|e| {
                serde_json::json!({
                    "name": e.name,
                    "url": e.url,
                    "date": e.date,
                })
            }).collect::<Vec<_>>(),
        }))
    }
    
    async fn health_check(&self) -> SourceResult<bool> {
        match self.base.http_client.get(&self.config.transparency_url).await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

// Helper function to create MrfFileInfo
fn create_mrf_file_info(
    file: FileEntry,
    file_type: MrfFileType,
    entity_name: &str,
    entity_type: &str,
    index_url: &str,
) -> MrfFileInfo {
    // Determine compression type from the file URL
    let compression = determine_compression_from_url(&file.location);
    
    MrfFileInfo {
        id: generate_file_id(&file.location),
        name: file.description.unwrap_or_else(|| match file_type {
            MrfFileType::InNetwork => "In-Network File".to_string(),
            MrfFileType::AllowedAmount => "Allowed Amount File".to_string(),
            _ => "MRF File".to_string(),
        }),
        url: file.location,
        file_type,
        size_bytes: None,
        last_modified: None,
        compression,
        metadata: serde_json::json!({
            "source": "united_health",
            "index_url": index_url,
            "reporting_entity_name": entity_name,
            "reporting_entity_type": entity_type,
        }),
    }
}

/// Determine compression type from URL
fn determine_compression_from_url(url: &str) -> Option<CompressionType> {
    let lower_url = url.to_lowercase();
    
    // Remove query parameters and fragments from URL for extension checking
    let path = lower_url.split('?').next().unwrap_or(&lower_url);
    let path = path.split('#').next().unwrap_or(path);
    
    if path.ends_with(".gz") || path.ends_with(".gzip") {
        Some(CompressionType::Gzip)
    } else if path.ends_with(".zip") {
        Some(CompressionType::Zip)
    } else if path.ends_with(".bz2") || path.ends_with(".bzip2") {
        Some(CompressionType::Bzip2)
    } else if path.ends_with(".json") {
        // Uncompressed JSON
        Some(CompressionType::None)
    } else {
        // For URLs without clear extensions, check for compression indicators in the path
        if path.contains("gzip") || path.contains(".gz") {
            Some(CompressionType::Gzip)
        } else if path.contains("zip") {
            Some(CompressionType::Zip)
        } else {
            // Default to None if we can't determine
            None
        }
    }
}

/// Entry for an index file found on the transparency page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexFileEntry {
    pub name: String,
    pub url: String,
    pub date: Option<DateTime<Utc>>,
}

/// Structure of a United Health index file
#[derive(Debug, Deserialize)]
struct IndexFile {
    reporting_entity_name: String,
    reporting_entity_type: String,
    reporting_structure: Vec<ReportingStructure>,
}

/// Reporting structure within an index file
#[derive(Debug, Deserialize)]
struct ReportingStructure {
    #[serde(rename = "in_network_files")]
    in_network_files: Option<Vec<FileEntry>>,
    
    #[serde(rename = "allowed_amount_files")]
    allowed_amount_files: Option<Vec<FileEntry>>,
    
    reporting_plans: Option<Vec<ReportingPlan>>,
}

/// Entry in an index file
#[derive(Debug, Deserialize)]
struct FileEntry {
    description: Option<String>,
    location: String,
}

/// Reporting plan information
#[derive(Debug, Deserialize)]
struct ReportingPlan {
    plan_id: String,
    plan_id_type: String,
    plan_market_type: String,
    plan_name: String,
}

/// Response from the blobs API endpoint
#[derive(Debug, Deserialize)]
struct BlobsApiResponse {
    blobs: Vec<BlobEntry>,
}

/// Entry in the blobs API response
#[derive(Debug, Deserialize)]
struct BlobEntry {
    name: String,
    #[serde(rename = "downloadUrl")]
    download_url: String,
    size: u64,
}

/// Extract date from filename (e.g., "2025-06-01_...")
fn extract_date_from_filename(filename: &str) -> Option<DateTime<Utc>> {
    let date_regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").ok()?;
    
    if let Some(captures) = date_regex.captures(filename) {
        let year = captures.get(1)?.as_str().parse().ok()?;
        let month = captures.get(2)?.as_str().parse().ok()?;
        let day = captures.get(3)?.as_str().parse().ok()?;
        
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(0, 0, 0))
            .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
    } else {
        None
    }
}

/// Generate a unique ID for a file based on its URL
fn generate_file_id(url: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    format!("uh_{:x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compression_detection() {
        // Test gzip detection
        assert_eq!(
            determine_compression_from_url("https://example.com/file.json.gz"),
            Some(CompressionType::Gzip)
        );
        assert_eq!(
            determine_compression_from_url("https://example.com/file.gzip"),
            Some(CompressionType::Gzip)
        );
        
        // Test zip detection
        assert_eq!(
            determine_compression_from_url("https://example.com/file.zip"),
            Some(CompressionType::Zip)
        );
        
        // Test bzip2 detection
        assert_eq!(
            determine_compression_from_url("https://example.com/file.bz2"),
            Some(CompressionType::Bzip2)
        );
        assert_eq!(
            determine_compression_from_url("https://example.com/file.bzip2"),
            Some(CompressionType::Bzip2)
        );
        
        // Test uncompressed JSON
        assert_eq!(
            determine_compression_from_url("https://example.com/file.json"),
            Some(CompressionType::None)
        );
        
        // Test URL with compression in path
        assert_eq!(
            determine_compression_from_url("https://example.com/gzip/data"),
            Some(CompressionType::Gzip)
        );
        
        // Test unknown format
        assert_eq!(
            determine_compression_from_url("https://example.com/file.dat"),
            None
        );
        
        // Test case insensitivity
        assert_eq!(
            determine_compression_from_url("https://example.com/FILE.JSON.GZ"),
            Some(CompressionType::Gzip)
        );
        
        // Test URLs with query parameters
        assert_eq!(
            determine_compression_from_url("https://example.com/file.json.gz?undefined"),
            Some(CompressionType::Gzip)
        );
        assert_eq!(
            determine_compression_from_url("https://mrfstorageprod.blob.core.windows.net/public-mrf/2025-06-01/2025-06-01_allowed-amounts.json.gz?undefined"),
            Some(CompressionType::Gzip)
        );
        assert_eq!(
            determine_compression_from_url("https://example.com/file.json?param=value&other=123"),
            Some(CompressionType::None)
        );
        
        // Test URLs with fragments
        assert_eq!(
            determine_compression_from_url("https://example.com/file.zip#section"),
            Some(CompressionType::Zip)
        );
        
        // Test URLs with both query params and fragments
        assert_eq!(
            determine_compression_from_url("https://example.com/file.bz2?param=1#section"),
            Some(CompressionType::Bzip2)
        );
    }
    
    #[test]
    fn test_date_extraction() {
        assert_eq!(
            extract_date_from_filename("2025-01-15_index.json"),
            Some(DateTime::from_naive_utc_and_offset(
                chrono::NaiveDate::from_ymd_opt(2025, 1, 15).unwrap()
                    .and_hms_opt(0, 0, 0).unwrap(),
                Utc
            ))
        );
        
        assert_eq!(
            extract_date_from_filename("prefix_2024-12-31_suffix.json"),
            Some(DateTime::from_naive_utc_and_offset(
                chrono::NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
                    .and_hms_opt(0, 0, 0).unwrap(),
                Utc
            ))
        );
        
        assert_eq!(
            extract_date_from_filename("no_date_here.json"),
            None
        );
    }
}

#[cfg(test)]
mod send_sync_tests {
    use super::*;
    fn assert_send_sync<T: Send + Sync>() {}
    #[test]
    fn united_health_source_is_send_sync() {
        assert_send_sync::<UnitedHealthSource>();
    }
}