//! Base implementation and utilities for MRF sources
//! 
//! This module provides common functionality that can be shared across
//! different insurer implementations, including HTTP client setup,
//! rate limiting, retry logic, and download utilities.

use super::{FetchOptions, MrfFileInfo, ProgressCallback, SourceConfig, SourceError, SourceResult};
use async_trait::async_trait;
use reqwest::{Client, ClientBuilder, Response};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use tracing::{debug, info, warn};

/// HTTP client wrapper with rate limiting and retry logic
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    config: SourceConfig,
}

impl HttpClient {
    /// Create a new HTTP client with the given configuration
    pub fn new(config: SourceConfig) -> SourceResult<Self> {
        let mut builder = ClientBuilder::new()
            .timeout(Duration::from_secs(
                config
                    .default_options
                    .as_ref()
                    .and_then(|o| o.timeout_secs)
                    .unwrap_or(300),
            ))
            .gzip(true)
            .deflate(true)
            .brotli(true)
            // Set high connection pool limits for maximum concurrency
            .pool_max_idle_per_host(10000)
            .pool_idle_timeout(Duration::from_secs(90))
            // Disable connection pooling limits
            .no_proxy()
            .tcp_nodelay(true)
            .http2_adaptive_window(true);

        if let Some(user_agent) = &config.user_agent {
            builder = builder.user_agent(user_agent);
        }

        let client = builder
            .build()
            .map_err(|e| SourceError::Config(format!("Failed to build HTTP client: {}", e)))?;

        Ok(Self {
            client,
            config,
        })
    }

    /// Execute an HTTP GET request with retry logic
    pub async fn get(&self, url: &str) -> SourceResult<Response> {
        let options = self.config.default_options.as_ref().cloned().unwrap_or_default();
        let max_retries = options.max_retries.unwrap_or(3);

        let mut attempt = 0;
        loop {
            debug!("HTTP GET attempt {} for {}", attempt + 1, url);

            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(response);
                    } else if response.status().as_u16() == 429 {
                        // Rate limited
                        let retry_after = response
                            .headers()
                            .get("retry-after")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| v.parse::<u64>().ok())
                            .unwrap_or(60);

                        warn!("Rate limited, retrying after {} seconds", retry_after);
                        return Err(SourceError::RateLimited(retry_after));
                    } else if attempt < max_retries && response.status().is_server_error() {
                        warn!(
                            "Server error ({}), retrying...",
                            response.status()
                        );
                        attempt += 1;
                        sleep(Duration::from_secs(2u64.pow(attempt))).await;
                        continue;
                    } else {
                        return Err(SourceError::Other(
                            format!("HTTP error: {}", response.status())
                        ));
                    }
                }
                Err(e) if attempt < max_retries => {
                    warn!("Request failed: {}, retrying...", e);
                    attempt += 1;
                    sleep(Duration::from_secs(2u64.pow(attempt))).await;
                    continue;
                }
                Err(e) => return Err(SourceError::Http(e)),
            }
        }
    }

    /// Download a file with progress tracking
    pub async fn download_file(
        &self,
        url: &str,
        path: &Path,
        progress: Option<ProgressCallback>,
    ) -> SourceResult<()> {
        let response = self.get(url).await?;
        
        let total_size = response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        if let Some(total) = total_size {
            info!("Downloading {} bytes to {:?}", total, path);
        }

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut file = File::create(path).await?;
        let mut downloaded = 0u64;
        let mut stream = response.bytes_stream();

        use futures_util::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(SourceError::Http)?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;

            if let (Some(callback), Some(total)) = (&progress, total_size) {
                callback(downloaded, total);
            }
        }

        file.flush().await?;
        info!("Download complete: {:?}", path);

        Ok(())
    }
}

/// Utility functions for MRF file handling
pub mod utils {
    use super::*;
    
    /// Detect MRF file type from URL or filename
    pub fn detect_file_type(url: &str) -> super::super::MrfFileType {
        let lower = url.to_lowercase();
        
        if lower.contains("table-of-contents") || lower.contains("toc") {
            super::super::MrfFileType::TableOfContents
        } else if lower.contains("in-network") || lower.contains("negotiated") {
            super::super::MrfFileType::InNetwork
        } else if lower.contains("allowed-amount") || lower.contains("out-of-network") {
            super::super::MrfFileType::AllowedAmount
        } else if lower.contains("provider-reference") || lower.contains("provider_reference") {
            super::super::MrfFileType::ProviderReference
        } else {
            super::super::MrfFileType::Unknown
        }
    }
    
    /// Detect compression type from URL or headers
    pub fn detect_compression(url: &str, content_type: Option<&str>) -> super::super::CompressionType {
        let lower = url.to_lowercase();
        
        if lower.ends_with(".gz") || lower.ends_with(".gzip") {
            super::super::CompressionType::Gzip
        } else if lower.ends_with(".zip") {
            super::super::CompressionType::Zip
        } else if lower.ends_with(".bz2") || lower.ends_with(".bzip2") {
            super::super::CompressionType::Bzip2
        } else if let Some(ct) = content_type {
            match ct {
                "application/gzip" | "application/x-gzip" => super::super::CompressionType::Gzip,
                "application/zip" => super::super::CompressionType::Zip,
                "application/x-bzip2" => super::super::CompressionType::Bzip2,
                _ => super::super::CompressionType::None,
            }
        } else {
            super::super::CompressionType::None
        }
    }
    
    /// Generate a cache key for a file
    pub fn cache_key(file_info: &MrfFileInfo) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        file_info.url.hash(&mut hasher);
        if let Some(modified) = &file_info.last_modified {
            modified.timestamp().hash(&mut hasher);
        }
        
        format!("{:x}", hasher.finish())
    }
    
    /// Get cache path for a file
    pub fn cache_path(cache_dir: &str, file_info: &MrfFileInfo) -> std::path::PathBuf {
        let key = cache_key(file_info);
        let extension = match file_info.compression {
            Some(super::super::CompressionType::Gzip) => "json.gz",
            Some(super::super::CompressionType::Zip) => "zip",
            Some(super::super::CompressionType::Bzip2) => "json.bz2",
            _ => "json",
        };
        
        std::path::Path::new(cache_dir)
            .join(&file_info.id)
            .join(format!("{}.{}", key, extension))
    }
}

/// Base implementation for common MRF source functionality
pub struct BaseSource {
    pub name: String,
    pub source_id: String,
    pub config: SourceConfig,
    pub http_client: HttpClient,
}

impl BaseSource {
    /// Create a new base source
    pub fn new(name: String, source_id: String, config: SourceConfig) -> SourceResult<Self> {
        let http_client = HttpClient::new(config.clone())?;
        
        Ok(Self {
            name,
            source_id,
            config,
            http_client,
        })
    }
    
    /// Check if a file is cached and still valid
    pub async fn check_cache(&self, file_info: &MrfFileInfo, options: &FetchOptions) -> Option<Vec<u8>> {
        if !options.use_cache {
            return None;
        }
        
        let cache_dir = options.cache_dir.as_ref()
            .or(self.config.default_options.as_ref()?.cache_dir.as_ref())?;
        
        let cache_path = utils::cache_path(cache_dir, file_info);
        
        // Use async metadata and read
        if tokio::fs::try_exists(&cache_path).await.ok()? {
            // Check if cache is still valid based on last_modified
            if let (Ok(metadata), Some(last_modified)) = (
                tokio::fs::metadata(&cache_path).await,
                file_info.last_modified
            ) {
                if let Ok(modified_time) = metadata.modified() {
                    let cache_time = modified_time
                        .duration_since(std::time::UNIX_EPOCH)
                        .ok()?
                        .as_secs();
                    
                    if cache_time > last_modified.timestamp() as u64 {
                        debug!("Using cached file: {:?}", cache_path);
                        return tokio::fs::read(&cache_path).await.ok();
                    }
                }
            }
        }
        
        None
    }
    
    /// Save file to cache
    pub async fn save_to_cache(
        &self,
        file_info: &MrfFileInfo,
        data: &[u8],
        options: &FetchOptions,
    ) -> SourceResult<()> {
        if !options.use_cache {
            return Ok(());
        }
        
        let cache_dir = options.cache_dir.as_ref()
            .or(self.config.default_options.as_ref().and_then(|o| o.cache_dir.as_ref()))
            .ok_or_else(|| SourceError::Config("No cache directory specified".to_string()))?;
        
        let cache_path = utils::cache_path(cache_dir, file_info);
        
        if let Some(parent) = cache_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        tokio::fs::write(&cache_path, data).await?;
        debug!("Saved to cache: {:?}", cache_path);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_file_type() {
        assert_eq!(
            utils::detect_file_type("https://example.com/table-of-contents.json"),
            super::super::MrfFileType::TableOfContents
        );
        assert_eq!(
            utils::detect_file_type("https://example.com/in-network-rates.json"),
            super::super::MrfFileType::InNetwork
        );
        assert_eq!(
            utils::detect_file_type("https://example.com/allowed-amounts.json"),
            super::super::MrfFileType::AllowedAmount
        );
    }
    
    #[test]
    fn test_detect_compression() {
        assert_eq!(
            utils::detect_compression("file.json.gz", None),
            super::super::CompressionType::Gzip
        );
        assert_eq!(
            utils::detect_compression("file.zip", None),
            super::super::CompressionType::Zip
        );
        assert_eq!(
            utils::detect_compression("file.json", Some("application/gzip")),
            super::super::CompressionType::Gzip
        );
    }
} 