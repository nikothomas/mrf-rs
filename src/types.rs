//! Core data types for MRF files according to Transparency in Coverage specifications

use serde::{Deserialize, Serialize};

/// Type of entity publishing the Machine-Readable File (MRF).
/// 
/// Represents the type of entity that is publishing the machine-readable file
/// according to the Transparency in Coverage final rules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityType {
    /// A group health plan
    #[serde(rename = "group health plan")]
    #[serde(alias = "Group Health Plan")]
    GroupHealthPlan,
    
    /// A health insurance issuer
    #[serde(rename = "health insurance issuer")]
    #[serde(alias = "Health Insurance Issuer")]
    HealthInsuranceIssuer,
    
    /// A third party with which the plan or issuer has contracted to provide
    /// the required information, such as a third-party administrator
    #[serde(rename = "third-party administrator")]
    #[serde(alias = "Third-Party Administrator")]
    #[serde(alias = "third party administrator")]
    ThirdPartyAdministrator,
    
    /// A health care claims clearinghouse
    #[serde(rename = "health care claims clearinghouse")]
    #[serde(alias = "Health Care Claims Clearinghouse")]
    HealthcareClearinghouse,
    
    /// An insurance company
    #[serde(rename = "insurer")]
    #[serde(alias = "Insurer")]
    Insurer,
    
    /// Any other entity type not explicitly listed
    #[serde(other)]
    Other,
}

/// Common billing code types used in healthcare.
/// 
/// Represents the various billing code standards that can be used for
/// negotiated rates for items and services.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BillingCodeType {
    /// Current Procedural Terminology - American Medical Association
    CPT,
    
    /// National Drug Code - FDA NDC
    NDC,
    
    /// Healthcare Common Procedural Coding System - CMS HCPCS
    HCPCS,
    
    /// Revenue Code
    RC,
    
    /// International Classification of Diseases
    ICD,
    
    /// Medicare Severity Diagnosis Related Groups - CMS DRGs
    #[serde(rename = "MS-DRG")]
    MSDRG,
    
    /// Refined Diagnosis Related Groups
    #[serde(rename = "R-DRG")]
    RDRG,
    
    /// Severity Diagnosis Related Groups
    #[serde(rename = "S-DRG")]
    SDRG,
    
    /// All Patient, Severity-Adjusted Diagnosis Related Groups
    #[serde(rename = "APS-DRG")]
    APSDRG,
    
    /// All Patient Diagnosis Related Groups
    #[serde(rename = "AP-DRG")]
    APDRG,
    
    /// All Patient Refined Diagnosis Related Groups - AHRQ documentation
    #[serde(rename = "APR-DRG")]
    APRDRG,
    
    /// Ambulatory Payment Classifications
    APC,
    
    /// Local Code Processing
    LOCAL,
    
    /// Enhanced Ambulatory Patient Grouping - 3M
    EAPG,
    
    /// Health Insurance Prospective Payment System - CMS
    HIPPS,
    
    /// Current Dental Terminology - ADA
    CDT,
    
    /// Custom Code Type: All - Represents all possible coding types under the contractual arrangement
    #[serde(rename = "CSTM-ALL")]
    CSTMALL,
    
    /// Any other billing code type not explicitly listed
    #[serde(other)]
    Other,
}

/// Type of negotiated rate arrangement.
/// 
/// Defines the different ways in which negotiated rates can be structured
/// between plans/issuers and providers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NegotiatedType {
    /// The negotiated rate, reflected as a dollar amount, for each covered item or service
    /// under the plan or coverage that the plan or issuer has contractually agreed to pay
    /// an in-network provider
    Negotiated,
    
    /// The price that a plan or issuer assigns to an item or service for the purpose
    /// of internal accounting, reconciliation with providers or submitting data
    Derived,
    
    /// The rate for a covered item or service from a particular in-network provider
    /// that a plan or issuer uses to determine a participant's cost-sharing liability
    #[serde(rename = "fee schedule")]
    FeeSchedule,
    
    /// The negotiated percentage value for a covered item or service from a particular
    /// in-network provider for a percentage of billed charges arrangement
    Percentage,
    
    /// The per diem daily rate, reflected as a dollar amount, for each covered item
    /// or service under the plan or coverage
    #[serde(rename = "per diem")]
    PerDiem,
}

/// Billing class for services.
/// 
/// Indicates whether the service is billed as professional, institutional, or both.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BillingClass {
    /// Professional services (e.g., physician services)
    Professional,
    
    /// Institutional services (e.g., hospital services)
    Institutional,
    
    /// Both professional and institutional
    #[serde(rename = "both")]
    Both,
}

/// Type of plan identifier.
/// 
/// Specifies whether the plan ID is an EIN or HIOS identifier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlanIdType {
    /// Employer Identification Number
    #[serde(rename = "EIN")]
    Ein,
    
    /// Health Insurance Oversight System identifier
    #[serde(rename = "HIOS")]
    Hios,
}

/// Market type for the health plan.
/// 
/// Indicates whether the plan is offered in the group or individual market.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MarketType {
    /// Group market (employer-sponsored plans)
    Group,
    
    /// Individual market
    Individual,
}

/// Type of negotiation arrangement.
/// 
/// Indicates whether a reimbursement arrangement other than a standard
/// fee-for-service model applies.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NegotiationArrangement {
    /// Fee-for-service arrangement
    Ffs,
    
    /// Bundled payment arrangement
    Bundle,
    
    /// Capitation arrangement
    Capitation,
}

/// Type of tax identifier.
/// 
/// Specifies whether the tax ID is an EIN or NPI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    /// Employer Identification Number issued by the IRS
    Ein,
    
    /// National Provider Identifier (used when SSN would otherwise be used)
    Npi,
}

// ============================================================================
// Table of Contents File Types
// ============================================================================

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

// ============================================================================
// Provider Reference File Types
// ============================================================================

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

// ============================================================================
// In-Network File Types
// ============================================================================

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

/// Provider group information.
/// 
/// Contains NPIs and TIN for a group of providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderGroup {
    /// Array of National Provider Identifiers (NPIs).
    /// Can contain a mix of Type 1 and Type 2 NPIs.
    /// Use [0] when NPIs are unknown at the TIN level.
    pub npi: Vec<i64>,
    
    /// Tax identification information for the provider group
    pub tin: TaxIdentifier,
}

/// Tax identifier.
/// 
/// Contains tax identification information for providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxIdentifier {
    /// Type of tax identifier (ein or npi)
    #[serde(rename = "type")]
    pub id_type: TaxIdType,
    
    /// The identifier value (EIN or NPI number)
    pub value: String,
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

// ============================================================================
// Out-of-Network Allowed Amount File Types
// ============================================================================

/// Out-of-Network Allowed Amount file structure.
/// 
/// Contains allowed amount information for out-of-network services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedAmountFile {
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
    
    /// Array of out-of-network allowed amounts
    pub out_of_network: Vec<OutOfNetworkRate>,
    
    /// The date in which the file was last updated (ISO 8601 format: YYYY-MM-DD)
    pub last_updated_on: String,
    
    /// The version of the schema for the produced information
    pub version: String,
    
    /// Source system identifier for the plan (optional custom field)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceSystem_plan")]
    pub source_system_plan: Option<String>,
}

/// Out-of-network rate information.
/// 
/// Contains information related to services provided out-of-network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutOfNetworkRate {
    /// Name of each item or service for which costs are payable
    pub name: String,
    
    /// Common billing code type
    pub billing_code_type: BillingCodeType,
    
    /// Version of the billing code type
    pub billing_code_type_version: String,
    
    /// The billing code for the item/service
    pub billing_code: String,
    
    /// Brief description of the item or service.
    /// For NDCs, must include proprietary and nonproprietary names.
    pub description: String,
    
    /// Array of allowed amounts
    pub allowed_amounts: Vec<AllowedAmount>,
}

/// Allowed amount for out-of-network services.
/// 
/// Documents the entity/business and service code where service was provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedAmount {
    /// Tax identification information for the place of business
    pub tin: TaxIdentifier,
    
    /// CMS-maintained two-digit place of service codes.
    /// Required when billing_class is "professional".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<Vec<String>>,
    
    /// Whether the service is professional or institutional
    pub billing_class: BillingClass,
    
    /// Array of payment information
    pub payments: Vec<Payment>,
}

/// Payment information.
/// 
/// Documents allowed amounts paid for out-of-network services.
/// Must represent at least 20 different claims to protect patient privacy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    /// The actual dollar amount the plan paid to the out-of-network provider
    /// plus the participant's share of the cost
    pub allowed_amount: f64,
    
    /// Billing code modifiers if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_code_modifier: Option<Vec<String>>,
    
    /// Array of providers who billed for this service
    pub providers: Vec<Provider>,
}

/// Provider information.
/// 
/// Defines NPIs and billed charges for out-of-network services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    /// Total dollar amount charged by the out-of-network provider
    pub billed_charge: f64,
    
    /// Array of provider NPIs
    pub npi: Vec<i64>,
}

// ============================================================================
// Unified MRF File Type (for backward compatibility)
// ============================================================================

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

// ============================================================================
// Utility Types
// ============================================================================

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