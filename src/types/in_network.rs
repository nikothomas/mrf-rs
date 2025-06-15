//! In-Network file types

use serde::{Deserialize, Serialize};
use super::common::{
    EntityType, PlanIdType, MarketType, NegotiationArrangement, 
    BillingCodeType, NegotiatedType, BillingClass, ProviderGroup
};

/// In-Network file structure.
/// 
/// Contains negotiated rate information for in-network providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InNetworkFile {
    /// The legal name of the entity publishing the machine-readable file
    pub reporting_entity_name: String,
    
    /// The type of entity that is publishing the machine-readable file
    pub reporting_entity_type: EntityType,
    
    /// The plan name and name of plan sponsor and/or insurance company.
    /// Required for single-plan files, optional for multi-plan files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    
    /// Type of plan identifier (EIN or HIOS).
    /// Required for single-plan files, optional for multi-plan files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id_type: Option<PlanIdType>,
    
    /// The plan identifier.
    /// Required for single-plan files, optional for multi-plan files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    
    /// Whether the plan is offered in the group or individual market.
    /// Required for single-plan files, optional for multi-plan files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_market_type: Option<MarketType>,
    
    /// Array of in-network negotiated rates
    pub in_network: Vec<InNetworkRate>,
    
    /// Array of provider reference objects for deduplication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_references: Option<Vec<ProviderReference>>,
    
    /// The date in which the file was last updated (ISO 8601 format: YYYY-MM-DD)
    pub last_updated_on: String,
    
    /// The version of the schema for the produced information
    pub version: String,
}

/// In-network rate information.
/// 
/// Defines an in-network rate for a specific item or service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InNetworkRate {
    /// Indication of the reimbursement arrangement (ffs, bundle, or capitation)
    pub negotiation_arrangement: NegotiationArrangement,
    
    /// Name of the item/service that is offered
    pub name: String,
    
    /// Common billing code type for the item/service
    pub billing_code_type: BillingCodeType,
    
    /// Version of the billing code type (e.g., "2023" for CPT codes)
    pub billing_code_type_version: String,
    
    /// The code used to identify health care items or services
    pub billing_code: String,
    
    /// Brief description of the item/service
    pub description: String,
    
    /// Array of negotiated rate details
    pub negotiated_rates: Vec<NegotiatedRateDetail>,
    
    /// Array of bundled codes if negotiation_arrangement is "bundle"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundled_codes: Option<Vec<BundledCode>>,
    
    /// Array of covered services if negotiation_arrangement is "capitation"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covered_services: Option<Vec<CoveredService>>,
}

/// Negotiated rate details.
/// 
/// Contains the negotiated prices and associated provider information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiatedRateDetail {
    /// Array of negotiated price objects
    pub negotiated_prices: Vec<NegotiatedPrice>,
    
    /// Array of provider groups (mutually exclusive with provider_references)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_groups: Option<Vec<ProviderGroup>>,
    
    /// Array of provider_group_ids referencing provider_references
    /// (mutually exclusive with provider_groups)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_references: Option<Vec<i32>>,
}

/// Negotiated price information.
/// 
/// Contains the negotiated pricing details for a specific arrangement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiatedPrice {
    /// Type of negotiated rate
    pub negotiated_type: NegotiatedType,
    
    /// The dollar amount or percentage based on negotiation_type.
    /// For percentage types, use whole numbers (e.g., 40.5 for 40.5%)
    pub negotiated_rate: f64,
    
    /// Date the agreement expires (ISO 8601 format: YYYY-MM-DD).
    /// Use "9999-12-31" for agreements with no expiration.
    pub expiration_date: String,
    
    /// Whether the service is professional, institutional, or both
    pub billing_class: BillingClass,
    
    /// CMS-maintained two-digit place of service codes.
    /// Required when billing_class is "professional".
    /// Use ["CSTM-00"] when rate applies to all service codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<Vec<String>>,
    
    /// Billing code modifiers (e.g., CPT modifiers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_code_modifier: Option<Vec<String>>,
    
    /// Additional context for negotiated arrangements that don't fit the schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
}

/// Bundled code information.
/// 
/// Contains codes that are part of a bundle arrangement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundledCode {
    /// Common billing code type
    pub billing_code_type: BillingCodeType,
    
    /// Version of the billing code type
    pub billing_code_type_version: String,
    
    /// The billing code
    pub billing_code: String,
    
    /// Brief description of the item/service
    pub description: String,
}

/// Covered service for capitation arrangements.
/// 
/// Contains services covered under a capitation arrangement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoveredService {
    /// Common billing code type
    pub billing_code_type: BillingCodeType,
    
    /// Version of the billing code type
    pub billing_code_type_version: String,
    
    /// The billing code
    pub billing_code: String,
    
    /// Brief description of the item/service
    pub description: String,
}

/// Provider reference for deduplication.
/// 
/// Used to reference provider groups defined elsewhere to reduce file size.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderReference {
    /// Unique primary key for the associated provider_group
    pub provider_group_id: i32,
    
    /// Provider groups (mutually exclusive with location)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_groups: Option<Vec<ProviderGroup>>,
    
    /// URL where provider group data can be downloaded
    /// (mutually exclusive with provider_groups)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
} 