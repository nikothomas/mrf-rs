//! Unified MRF file type and utility types

use serde::{Deserialize, Serialize};
use super::{
    TableOfContentsFile, InNetworkFile, AllowedAmountFile, ProviderReferenceFile
};

/// Generic MRF file that can represent any of the file types.
/// 
/// Used for parsing when the specific file type is unknown.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MrfFile {
    /// Table of Contents file mapping plans to their MRF files
    TableOfContents(TableOfContentsFile),
    
    /// In-Network file with negotiated rates
    InNetwork(InNetworkFile),
    
    /// Out-of-Network Allowed Amount file
    AllowedAmount(AllowedAmountFile),
    
    /// Provider Reference file for deduplication
    ProviderReference(ProviderReferenceFile),
}

/// Processing statistics for MRF file operations.
/// 
/// Tracks various metrics during file processing.
#[derive(Debug, Default)]
pub struct ProcessingStats {
    /// Total number of records processed
    pub total_records: usize,
    
    /// Number of in-network rates processed
    pub in_network_rates: usize,
    
    /// Number of out-of-network rates processed
    pub out_of_network_rates: usize,
    
    /// Number of providers processed
    pub providers_processed: usize,
    
    /// Number of errors encountered during processing
    pub errors_encountered: usize,
    
    /// Total processing time in seconds
    pub processing_time_secs: u64,
    
    /// Size of the processed file in bytes
    pub file_size_bytes: u64,
} 