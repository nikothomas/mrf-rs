//! Table of Contents file types

use serde::{Deserialize, Serialize};
use super::common::{EntityType, PlanIdType, MarketType};

/// Table of Contents file structure.
/// 
/// An optional file that can be leveraged to significantly decrease file sizes
/// of the required machine-readable files. The Transparency in Coverage final
/// rules do not require plans and issuers to publish a Table of Contents file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableOfContentsFile {
    /// The legal name of the entity publishing the machine-readable file
    pub reporting_entity_name: String,
    
    /// The type of entity that is publishing the machine-readable file
    pub reporting_entity_type: EntityType,
    
    /// An array of reporting structures that map plans to their files
    pub reporting_structure: Vec<ReportingStructure>,
    
    /// The version of the schema for the produced information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Reporting structure for table of contents.
/// 
/// Maps associated plans to their in-network and allowed amount files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingStructure {
    /// An array of plans associated with the files
    pub reporting_plans: Vec<ReportingPlan>,
    
    /// An array of file locations for in-network files.
    /// At least one of `in_network_files` or `allowed_amount_file` must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_network_files: Option<Vec<FileLocation>>,
    
    /// The file location for the allowed amounts file.
    /// At least one of `in_network_files` or `allowed_amount_file` must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_amount_file: Option<FileLocation>,
}

/// Reporting plan information.
/// 
/// Contains the plan details for plans included in the reporting structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingPlan {
    /// The plan name and name of plan sponsor and/or insurance company
    pub plan_name: String,
    
    /// Type of plan identifier (EIN or HIOS)
    pub plan_id_type: PlanIdType,
    
    /// The 10-digit HIOS identifier, or if not available, the 5-digit HIOS identifier,
    /// or if no HIOS identifier is available, the EIN for each plan or coverage
    pub plan_id: String,
    
    /// Whether the plan is offered in the group or individual market
    pub plan_market_type: MarketType,
}

/// File location information.
/// 
/// Contains the description and URL for a machine-readable file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLocation {
    /// Description of the file included
    pub description: String,
    
    /// A fully qualified domain name where the file can be downloaded.
    /// Must be an HTTPS URL.
    pub location: String,
} 