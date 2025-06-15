//! Integration tests for parsing real MRF files

use mrf_rs::parser::MrfParser;
use mrf_rs::types::*;
use std::path::Path;

#[test]
fn test_parse_all_savers_mrf_file() {
    // Path to the test asset file
    let test_file_path = Path::new("tests/in-network-rates/2025-06-01_ALL-SAVERS-INSURANCE-COMPANY_Insurer_CMC_Transplant_MRRF_in-network-rates.json");
    
    // Ensure the file exists
    assert!(test_file_path.exists(), "Test file not found at: {:?}", test_file_path);
    
    // Parse the file
    let result = MrfParser::parse_file(test_file_path);
    
    // Assert parsing succeeded
    assert!(result.is_ok(), "Failed to parse MRF file: {:?}", result.err());
    
    match result.unwrap() {
        MrfFile::InNetwork(mrf_file) => {
            // Validate basic structure
            assert_eq!(mrf_file.reporting_entity_name, "ALL SAVERS INSURANCE COMPANY");
            assert_eq!(mrf_file.reporting_entity_type, EntityType::Insurer);
            
            // Check that we have in-network rates
            assert!(!mrf_file.in_network.is_empty(), "Expected at least one in-network rate");
            
            // Validate the first in-network rate has required fields
            let first_rate = &mrf_file.in_network[0];
            assert!(!first_rate.name.is_empty());
            assert!(!first_rate.billing_code.is_empty());
            assert!(!first_rate.billing_code_type_version.is_empty());
            
            // Check for negotiated rates
            assert!(!first_rate.negotiated_rates.is_empty(), "Expected negotiated rates to be present");
            
            // Validate negotiated prices exist
            for rate_detail in &first_rate.negotiated_rates {
                assert!(!rate_detail.negotiated_prices.is_empty(), "Expected negotiated prices");
                
                // Validate each negotiated price
                for price in &rate_detail.negotiated_prices {
                    assert!(price.negotiated_rate >= 0.0, "Negotiated rate should be non-negative");
                    assert!(!price.expiration_date.is_empty(), "Expiration date should be present");
                }
            }
            
            println!("Successfully parsed MRF file with {} in-network rates", mrf_file.in_network.len());
        }
        _ => panic!("Expected InNetwork file type"),
    }
}

#[test]
fn test_parse_allowed_amounts_file() {
    let test_file_path = Path::new("tests/allowed-amounts/2025-06-01_10-ROADS-EXPRESS-LLC_10-ROADS-EXPRESS-LLC_allowed-amounts.json");
    
    if !test_file_path.exists() {
        eprintln!("Skipping allowed amounts test - file not found");
        return;
    }
    
    let result = MrfParser::parse_file(test_file_path);
    assert!(result.is_ok(), "Failed to parse allowed amounts file: {:?}", result.err());
    
    match result.unwrap() {
        MrfFile::AllowedAmount(file) => {
            assert_eq!(file.reporting_entity_name, "Surest");
            assert_eq!(file.reporting_entity_type, EntityType::ThirdPartyAdministrator);
            assert_eq!(file.version, "1.0.0");
            assert_eq!(file.last_updated_on, "2025-06-01");
            assert!(file.out_of_network.is_empty(), "Expected empty out_of_network array");
            
            println!("Successfully parsed allowed amounts file");
        }
        _ => panic!("Expected AllowedAmount file type"),
    }
}

#[test]
fn test_parse_table_of_contents_file() {
    let test_file_path = Path::new("tests/table-of-contents/2025-06-01_1-800-RADIATOR-OF-DALLAS-FORT-WORTH-LLC_index.json");
    
    if !test_file_path.exists() {
        eprintln!("Skipping table of contents test - file not found");
        return;
    }
    
    let result = MrfParser::parse_file(test_file_path);
    assert!(result.is_ok(), "Failed to parse table of contents file: {:?}", result.err());
    
    match result.unwrap() {
        MrfFile::TableOfContents(file) => {
            assert_eq!(file.reporting_entity_name, "United-HealthCare-Services-Inc");
            assert_eq!(file.reporting_entity_type, EntityType::ThirdPartyAdministrator);
            assert!(file.version.is_none(), "Expected no version field in this file");
            assert!(!file.reporting_structure.is_empty(), "Expected reporting structure");
            
            let first_structure = &file.reporting_structure[0];
            assert!(!first_structure.reporting_plans.is_empty(), "Expected reporting plans");
            
            let first_plan = &first_structure.reporting_plans[0];
            assert_eq!(first_plan.plan_name, "United-Healthcare-EPO");
            assert_eq!(first_plan.plan_id, "510540405");
            assert_eq!(first_plan.plan_id_type, PlanIdType::Ein);
            assert_eq!(first_plan.plan_market_type, MarketType::Group);
            
            assert!(first_structure.in_network_files.is_some(), "Expected in_network_files");
            let in_network_files = first_structure.in_network_files.as_ref().unwrap();
            assert_eq!(in_network_files.len(), 5, "Expected 5 in-network files");
            
            println!("Successfully parsed table of contents file with {} reporting structures", 
                     file.reporting_structure.len());
        }
        _ => panic!("Expected TableOfContents file type"),
    }
}

#[test]
fn test_parse_large_file_performance() {
    use std::time::Instant;
    
    let test_file_path = Path::new("tests/in-network-rates/2025-06-01_ALL-SAVERS-INSURANCE-COMPANY_Insurer_CMC_Transplant_MRRF_in-network-rates.json");
    
    if !test_file_path.exists() {
        eprintln!("Skipping performance test - file not found");
        return;
    }
    
    let start = Instant::now();
    let result = MrfParser::parse_file(test_file_path);
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "Failed to parse file");
    
    println!("Parsed 4.3MB MRF file in {:?}", duration);
    
    // Ensure parsing doesn't take too long (adjust threshold as needed)
    assert!(duration.as_secs() < 30, "Parsing took too long: {:?}", duration);
} 