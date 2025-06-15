//! Out-of-Network Allowed Amount file types

use serde::{Deserialize, Serialize};
use super::common::{
    EntityType, PlanIdType, MarketType, BillingCodeType, 
    BillingClass, TaxIdentifier
};

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