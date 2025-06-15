//! Provider Reference file types

use serde::{Deserialize, Serialize};
use super::common::ProviderGroup;

/// Provider Reference file structure.
/// 
/// An optional file that can be leveraged to significantly decrease file sizes
/// of the required machine-readable files by deduplicating provider information.
/// The Transparency in Coverage final rules do not require plans and issuers
/// to publish a Provider Reference file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderReferenceFile {
    /// Array of provider groups with their NPIs and TINs
    pub provider_groups: Vec<ProviderGroup>,
    
    /// The version of the schema for the produced information
    pub version: String,
} 