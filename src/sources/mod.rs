//! Sources module for fetching MRF data from various insurers
//! 
//! This module provides a standardized interface for discovering and downloading
//! MRF files from different insurance companies. Each insurer has its own
//! implementation that handles their specific file organization and formats.
//! 
//! # Architecture
//! 
//! The module is organized as follows:
//! - Core traits and types (this file)
//! - Base implementation with common functionality
//! - Insurer-specific implementations
//! 
//! # Examples
//! 
//! ```no_run
//! use mrf_rs::sources::{MrfSource, SourceResult};
//! use mrf_rs::sources::anthem::AnthemSource;
//! 
//! # async fn example() -> SourceResult<()> {
//! // Create a source for a specific insurer
//! let source = AnthemSource::new();
//! 
//! // Discover available MRF files
//! let files = source.discover_files().await?;
//! 
//! // Download a specific file
//! let mrf_data = source.fetch_file(&files[0]).await?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

pub mod base;
pub mod united_health;

// Re-export insurer modules when they're implemented
// pub mod aetna;
// pub mod cigna;

/// Error type for source operations
#[derive(Debug, Error)]
pub enum SourceError {
    /// HTTP request failed
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    /// IO error occurred
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Parsing error
    #[error("Parse error: {0}")]
    Parse(String),
    
    /// File not found
    #[error("File not found: {0}")]
    NotFound(String),
    
    /// Rate limit exceeded
    #[error("Rate limit exceeded, retry after {0} seconds")]
    RateLimited(u64),
    
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),
    
    /// Generic source error
    #[error("Source error: {0}")]
    Other(String),
}

/// Result type for source operations
pub type SourceResult<T> = Result<T, SourceError>;

/// Information about an available MRF file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MrfFileInfo {
    /// Unique identifier for this file
    pub id: String,
    
    /// Human-readable name/description
    pub name: String,
    
    /// URL where the file can be downloaded
    pub url: String,
    
    /// Type of MRF file
    pub file_type: MrfFileType,
    
    /// Size in bytes (if known)
    pub size_bytes: Option<u64>,
    
    /// Last modified date (if known)
    pub last_modified: Option<DateTime<Utc>>,
    
    /// Compression format (if any)
    pub compression: Option<CompressionType>,
    
    /// Additional metadata specific to the source
    pub metadata: serde_json::Value,
}

/// Type of MRF file
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MrfFileType {
    /// Table of Contents file
    TableOfContents,
    
    /// In-Network rates file
    InNetwork,
    
    /// Out-of-Network allowed amounts file
    AllowedAmount,
    
    /// Provider reference file
    ProviderReference,
    
    /// Unknown or mixed content
    Unknown,
}

impl MrfFileType {
    /// Convert the file type to a string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            MrfFileType::TableOfContents => "toc",
            MrfFileType::InNetwork => "in_network",
            MrfFileType::AllowedAmount => "allowed_amount",
            MrfFileType::ProviderReference => "provider_ref",
            MrfFileType::Unknown => "unknown",
        }
    }
}

/// Compression type for MRF files
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressionType {
    /// Gzip compression
    Gzip,
    
    /// Zip archive
    Zip,
    
    /// Bzip2 compression
    Bzip2,
    
    /// No compression
    None,
}

/// Options for fetching MRF files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchOptions {
    /// Maximum file size to download (in bytes)
    pub max_size: Option<u64>,
    
    /// Whether to use local cache
    pub use_cache: bool,
    
    /// Cache directory path
    pub cache_dir: Option<String>,
    
    /// Request timeout in seconds
    pub timeout_secs: Option<u64>,
    
    /// Number of retry attempts
    pub max_retries: Option<u32>,
    
    /// Whether to verify SSL certificates
    pub verify_ssl: bool,
}

impl Default for FetchOptions {
    fn default() -> Self {
        Self {
            max_size: None,
            use_cache: true,
            cache_dir: None,
            timeout_secs: Some(300), // 5 minutes default
            max_retries: Some(3),
            verify_ssl: true,
        }
    }
}

/// Progress callback for download operations
pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send + Sync>;

/// Main trait for MRF data sources
#[async_trait]
pub trait MrfSource: Send + Sync {
    /// Get the name of this source (e.g., "Anthem", "Aetna")
    fn name(&self) -> &str;
    
    /// Get the base URL or identifier for this source
    fn source_id(&self) -> &str;
    
    /// Discover available MRF files from this source
    /// 
    /// This method should return a list of all available MRF files
    /// without downloading them. The implementation varies by insurer.
    async fn discover_files(&self) -> SourceResult<Vec<MrfFileInfo>>;
    
    /// Fetch a specific MRF file
    /// 
    /// Downloads and returns the raw content of an MRF file.
    /// For large files, consider using `fetch_file_to_path` instead.
    async fn fetch_file(
        &self,
        file_info: &MrfFileInfo,
        options: Option<FetchOptions>,
    ) -> SourceResult<Vec<u8>>;
    
    /// Fetch an MRF file and save it to a path
    /// 
    /// More efficient for large files as it streams directly to disk.
    async fn fetch_file_to_path(
        &self,
        file_info: &MrfFileInfo,
        path: &Path,
        options: Option<FetchOptions>,
        progress: Option<ProgressCallback>,
    ) -> SourceResult<()>;
    
    /// Get metadata about available files without full discovery
    /// 
    /// Some sources may provide a summary or table of contents
    /// that can be fetched more quickly than full discovery.
    async fn get_metadata(&self) -> SourceResult<serde_json::Value> {
        Ok(serde_json::json!({
            "source": self.name(),
            "discovery_required": true
        }))
    }
    
    /// Check if the source is currently available
    async fn health_check(&self) -> SourceResult<bool> {
        Ok(true)
    }
}

/// Configuration for source implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    /// Base URL for the insurer's MRF files
    pub base_url: String,
    
    /// User agent string for HTTP requests
    pub user_agent: Option<String>,
    
    /// Rate limit (requests per second)
    pub rate_limit: Option<f64>,
    
    /// Default fetch options
    pub default_options: Option<FetchOptions>,
    
    /// Additional source-specific configuration
    pub extra: serde_json::Value,
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            user_agent: Some("mrf-rs/0.1.0".to_string()),
            rate_limit: Some(100.0), // 100 requests per second
            default_options: Some(FetchOptions::default()),
            extra: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_fetch_options() {
        let options = FetchOptions::default();
        assert!(options.use_cache);
        assert!(options.verify_ssl);
        assert_eq!(options.timeout_secs, Some(300));
        assert_eq!(options.max_retries, Some(3));
    }
    
    #[test]
    fn test_source_config_default() {
        let config = SourceConfig::default();
        assert_eq!(config.user_agent, Some("mrf-rs/0.1.0".to_string()));
        assert_eq!(config.rate_limit, Some(100.0));
    }
}
