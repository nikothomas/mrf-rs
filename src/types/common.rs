//! Common types shared across all MRF file formats

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