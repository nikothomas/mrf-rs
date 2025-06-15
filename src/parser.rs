//! Parser for MRF (Machine-Readable Files) JSON files
//! 
//! This module provides a comprehensive parser for handling various types of MRF files
//! as defined by the CMS (Centers for Medicare & Medicaid Services) transparency in
//! coverage regulations. The parser supports all four main MRF file types:
//! 
//! - Table of Contents files
//! - In-Network files
//! - Allowed Amount files
//! - Provider Reference files
//! 
//! # Features
//! 
//! - Type-safe parsing with automatic file type detection
//! - Support for multiple input sources (files, readers, strings, bytes)
//! - Comprehensive error handling with detailed error messages
//! - Memory-efficient streaming for large files
//! - Generic parsing capabilities for custom types
//! 
//! # Examples
//! 
//! ## Basic Usage
//! 
//! ```no_run
//! use mrf_rs::parser::MrfParser;
//! 
//! // Parse any MRF file type automatically
//! let mrf_file = MrfParser::parse_file("path/to/mrf.json")?;
//! 
//! // Parse from a string
//! let json_str = r#"{"reporting_entity_name": "Example", ...}"#;
//! let mrf_file = MrfParser::parse_str(json_str)?;
//! # Ok::<(), mrf_rs::parser::ParseError>(())
//! ```
//! 
//! ## Type-Specific Parsing
//! 
//! ```no_run
//! use mrf_rs::parser::MrfParser;
//! 
//! // Parse a specific file type when you know what to expect
//! let in_network = MrfParser::parse_in_network_file("in_network.json")?;
//! let toc = MrfParser::parse_table_of_contents_file("toc.json")?;
//! # Ok::<(), mrf_rs::parser::ParseError>(())
//! ```

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use serde::de::DeserializeOwned;
use serde_json;

use crate::types::{MrfFile, TableOfContentsFile, InNetworkFile, AllowedAmountFile, ProviderReferenceFile};

/// Error type for parsing operations
/// 
/// This enum represents all possible errors that can occur during MRF file parsing.
/// It provides detailed error messages and preserves the underlying error chain
/// for debugging purposes.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// IO error occurred while reading the file
    /// 
    /// This typically happens when:
    /// - The file cannot be opened due to permissions
    /// - The file is corrupted or unreadable
    /// - Network issues when reading from remote sources
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON parsing error occurred
    /// 
    /// This error indicates that the file content is not valid JSON or
    /// doesn't match the expected MRF schema. The error message includes
    /// details about what went wrong and where in the JSON structure.
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// The specified file was not found
    /// 
    /// This error is returned when attempting to parse a file that doesn't
    /// exist at the specified path. The error message includes the path
    /// that was attempted.
    #[error("File not found: {0}")]
    FileNotFound(String),
}

/// Result type alias for parsing operations
/// 
/// This type alias simplifies the return type signatures throughout the parser
/// module. All parsing methods return this type, making error handling consistent.
pub type ParseResult<T> = Result<T, ParseError>;

/// Main parser struct for MRF files
/// 
/// `MrfParser` provides a collection of static methods for parsing MRF files
/// from various sources. It automatically detects the file type based on the
/// JSON structure and returns the appropriate typed result.
/// 
/// # Design Philosophy
/// 
/// The parser is designed with the following principles:
/// 
/// - **Zero-copy where possible**: Uses streaming parsers to minimize memory usage
/// - **Type safety**: Returns strongly-typed structures that match the MRF schema
/// - **Flexibility**: Supports multiple input sources and formats
/// - **Error recovery**: Provides detailed error messages for debugging
/// 
/// # Performance Considerations
/// 
/// For large MRF files (which can be several GB), consider using the reader-based
/// methods rather than loading the entire file into memory. The parser uses
/// buffered readers internally to optimize performance.
pub struct MrfParser;

impl MrfParser {
    /// Parse any MRF file type from a file path
    /// 
    /// This is the primary entry point for parsing MRF files. It automatically
    /// detects the file type based on the JSON structure and returns the
    /// appropriate `MrfFile` variant.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The file path to parse. Can be relative or absolute.
    /// 
    /// # Returns
    /// 
    /// Returns a `ParseResult<MrfFile>` which will be one of:
    /// - `MrfFile::TableOfContents` for table of contents files
    /// - `MrfFile::InNetwork` for in-network rate files
    /// - `MrfFile::AllowedAmount` for allowed amount files
    /// - `MrfFile::ProviderReference` for provider reference files
    /// 
    /// # Errors
    /// 
    /// - `ParseError::FileNotFound` if the file doesn't exist
    /// - `ParseError::Io` if the file cannot be read
    /// - `ParseError::Json` if the file is not valid JSON or doesn't match the MRF schema
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use mrf_rs::parser::MrfParser;
    /// use mrf_rs::types::MrfFile;
    /// 
    /// let result = MrfParser::parse_file("data/in_network.json")?;
    /// 
    /// match result {
    ///     MrfFile::InNetwork(file) => {
    ///         println!("Parsed in-network file from: {}", file.reporting_entity_name);
    ///     }
    ///     MrfFile::TableOfContents(file) => {
    ///         println!("Parsed TOC with {} entries", file.reporting_structure.len());
    ///     }
    ///     _ => println!("Parsed other MRF file type"),
    /// }
    /// # Ok::<(), mrf_rs::parser::ParseError>(())
    /// ```
    /// 
    /// # Performance Note
    /// 
    /// This method reads the entire file into memory before parsing. For very large
    /// files, consider using memory-mapped files or streaming approaches.
    pub fn parse_file<P: AsRef<Path>>(path: P) -> ParseResult<MrfFile> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(ParseError::FileNotFound(
                path.to_string_lossy().to_string()
            ));
        }
        
        let mut file = File::open(path)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;
        let reader = BufReader::new(file_content.as_bytes());
        
        Self::parse_reader(reader)
    }
    
    /// Parse any MRF file type from a reader
    /// 
    /// This method is useful when you have the MRF data from a source other than
    /// a file, such as a network stream, compressed archive, or in-memory buffer.
    /// 
    /// # Arguments
    /// 
    /// * `reader` - Any type that implements `Read`, such as `File`, `Cursor`, or network streams
    /// 
    /// # Returns
    /// 
    /// Returns the parsed `MrfFile` with automatic type detection
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::io::Cursor;
    /// use mrf_rs::parser::MrfParser;
    /// 
    /// let json_data = r#"{
    ///     "reporting_entity_name": "Example Corp",
    ///     "reporting_entity_type": "health insurance issuer",
    ///     "in_network": [],
    ///     "last_updated_on": "2024-01-01",
    ///     "version": "1.0.0"
    /// }"#;
    /// 
    /// let cursor = Cursor::new(json_data.as_bytes());
    /// let result = MrfParser::parse_reader(cursor)?;
    /// # Ok::<(), mrf_rs::parser::ParseError>(())
    /// ```
    /// 
    /// # Memory Efficiency
    /// 
    /// This method uses `serde_json`'s streaming parser, which is more memory
    /// efficient than loading the entire file into a string first.
    pub fn parse_reader<R: Read>(reader: R) -> ParseResult<MrfFile> {
        let mrf_file = serde_json::from_reader(reader)?;
        Ok(mrf_file)
    }
    
    /// Parse JSON from a string
    /// 
    /// Convenient method for parsing MRF data that's already loaded as a string.
    /// This is useful for testing, small files, or when the data comes from an API.
    /// 
    /// # Arguments
    /// 
    /// * `json_str` - A string containing valid MRF JSON data
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mrf_rs::parser::MrfParser;
    /// use mrf_rs::types::MrfFile;
    /// 
    /// let json = r#"{
    ///     "provider_groups": [],
    ///     "version": "1.0.0"
    /// }"#;
    /// 
    /// let result = MrfParser::parse_str(json)?;
    /// assert!(matches!(result, MrfFile::ProviderReference(_)));
    /// # Ok::<(), mrf_rs::parser::ParseError>(())
    /// ```
    pub fn parse_str(json_str: &str) -> ParseResult<MrfFile> {
        let mrf_file = serde_json::from_str(json_str)?;
        Ok(mrf_file)
    }
    
    /// Parse JSON from bytes
    /// 
    /// Parse MRF data from a byte slice. This is useful when working with
    /// binary data or when the encoding is not guaranteed to be valid UTF-8.
    /// 
    /// # Arguments
    /// 
    /// * `json_bytes` - A byte slice containing MRF JSON data
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mrf_rs::parser::MrfParser;
    /// 
    /// let json_bytes = br#"{"provider_groups": [], "version": "1.0.0"}"#;
    /// let result = MrfParser::parse_bytes(json_bytes)?;
    /// # Ok::<(), mrf_rs::parser::ParseError>(())
    /// ```
    pub fn parse_bytes(json_bytes: &[u8]) -> ParseResult<MrfFile> {
        let mrf_file = serde_json::from_slice(json_bytes)?;
        Ok(mrf_file)
    }
    
    // Specific parsers for each file type
    
    /// Parse a Table of Contents file specifically
    /// 
    /// Use this method when you know the file is a Table of Contents file and
    /// want to get a `TableOfContentsFile` directly without pattern matching.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the Table of Contents JSON file
    /// 
    /// # Returns
    /// 
    /// Returns a strongly-typed `TableOfContentsFile` structure
    /// 
    /// # Errors
    /// 
    /// Returns an error if the file is not a valid Table of Contents file,
    /// even if it's a valid MRF file of another type.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use mrf_rs::parser::MrfParser;
    /// 
    /// let toc = MrfParser::parse_table_of_contents_file("toc.json")?;
    /// 
    /// for entry in &toc.reporting_structure {
    ///     println!("Found {} in-network files", entry.in_network_files.len());
    /// }
    /// # Ok::<(), mrf_rs::parser::ParseError>(())
    /// ```
    pub fn parse_table_of_contents_file<P: AsRef<Path>>(path: P) -> ParseResult<TableOfContentsFile> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(ParseError::FileNotFound(
                path.to_string_lossy().to_string()
            ));
        }
        
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        Self::parse_table_of_contents_reader(reader)
    }
    
    /// Parse a Table of Contents from a reader
    /// 
    /// Reader-based version of `parse_table_of_contents_file` for more flexibility
    /// in data sources.
    /// 
    /// # Type Safety
    /// 
    /// This method will fail if the JSON doesn't match the Table of Contents schema,
    /// providing clear error messages about what fields are missing or incorrect.
    pub fn parse_table_of_contents_reader<R: Read>(reader: R) -> ParseResult<TableOfContentsFile> {
        let toc_file = serde_json::from_reader(reader)?;
        Ok(toc_file)
    }
    
    /// Parse an In-Network file specifically
    /// 
    /// Use this method when you know the file is an In-Network rate file and
    /// want direct access to the typed structure.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the In-Network JSON file
    /// 
    /// # In-Network File Structure
    /// 
    /// In-Network files contain negotiated rates between providers and insurance
    /// plans. They can be very large (multiple GB) as they contain rates for
    /// every procedure code and provider combination.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use mrf_rs::parser::MrfParser;
    /// 
    /// let in_network = MrfParser::parse_in_network_file("in_network.json")?;
    /// 
    /// println!("File last updated: {}", in_network.last_updated_on);
    /// println!("Number of items: {}", in_network.in_network.len());
    /// # Ok::<(), mrf_rs::parser::ParseError>(())
    /// ```
    pub fn parse_in_network_file<P: AsRef<Path>>(path: P) -> ParseResult<InNetworkFile> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(ParseError::FileNotFound(
                path.to_string_lossy().to_string()
            ));
        }
        
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        Self::parse_in_network_reader(reader)
    }
    
    /// Parse an In-Network file from a reader
    /// 
    /// Reader-based version for parsing In-Network files from various sources.
    /// 
    /// # Performance Considerations
    /// 
    /// In-Network files can be extremely large. Consider using streaming
    /// approaches or processing the file in chunks for production use.
    pub fn parse_in_network_reader<R: Read>(reader: R) -> ParseResult<InNetworkFile> {
        let in_network_file = serde_json::from_reader(reader)?;
        Ok(in_network_file)
    }
    
    /// Parse an Allowed Amount file specifically
    /// 
    /// Use this method for parsing out-of-network allowed amount files, which
    /// contain historical payment data for services provided outside the network.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the Allowed Amount JSON file
    /// 
    /// # Allowed Amount Files
    /// 
    /// These files contain billed charges and allowed amounts for out-of-network
    /// services, helping consumers understand typical costs for services outside
    /// their insurance network.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use mrf_rs::parser::MrfParser;
    /// 
    /// let allowed = MrfParser::parse_allowed_amount_file("allowed_amounts.json")?;
    /// 
    /// for item in &allowed.out_of_network {
    ///     println!("Service: {:?}", item.billing_code);
    /// }
    /// # Ok::<(), mrf_rs::parser::ParseError>(())
    /// ```
    pub fn parse_allowed_amount_file<P: AsRef<Path>>(path: P) -> ParseResult<AllowedAmountFile> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(ParseError::FileNotFound(
                path.to_string_lossy().to_string()
            ));
        }
        
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        Self::parse_allowed_amount_reader(reader)
    }
    
    /// Parse an Allowed Amount file from a reader
    /// 
    /// Reader-based version for flexible data source handling.
    pub fn parse_allowed_amount_reader<R: Read>(reader: R) -> ParseResult<AllowedAmountFile> {
        let allowed_amount_file = serde_json::from_reader(reader)?;
        Ok(allowed_amount_file)
    }
    
    /// Parse a Provider Reference file specifically
    /// 
    /// Provider Reference files contain detailed information about healthcare
    /// providers referenced in other MRF files.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the Provider Reference JSON file
    /// 
    /// # Provider Reference Structure
    /// 
    /// These files map provider IDs to actual provider information including:
    /// - NPI (National Provider Identifier)
    /// - TIN (Tax Identification Number)
    /// - Provider names and addresses
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use mrf_rs::parser::MrfParser;
    /// 
    /// let providers = MrfParser::parse_provider_reference_file("providers.json")?;
    /// 
    /// for group in &providers.provider_groups {
    ///     println!("Provider group with {} NPIs", group.npi.len());
    /// }
    /// # Ok::<(), mrf_rs::parser::ParseError>(())
    /// ```
    pub fn parse_provider_reference_file<P: AsRef<Path>>(path: P) -> ParseResult<ProviderReferenceFile> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(ParseError::FileNotFound(
                path.to_string_lossy().to_string()
            ));
        }
        
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        Self::parse_provider_reference_reader(reader)
    }
    
    /// Parse a Provider Reference file from a reader
    /// 
    /// Reader-based parsing for Provider Reference files.
    pub fn parse_provider_reference_reader<R: Read>(reader: R) -> ParseResult<ProviderReferenceFile> {
        let provider_ref_file = serde_json::from_reader(reader)?;
        Ok(provider_ref_file)
    }
    
    /// Generic parser for any type that implements DeserializeOwned
    /// 
    /// This method provides flexibility to parse specific parts of MRF files
    /// or custom structures that conform to the MRF schema.
    /// 
    /// # Type Parameters
    /// 
    /// * `T` - The target type to deserialize into. Must implement `DeserializeOwned`.
    /// * `R` - The reader type. Must implement `Read`.
    /// 
    /// # Use Cases
    /// 
    /// - Parsing only specific sections of large MRF files
    /// - Working with custom types that extend the standard MRF schema
    /// - Building specialized parsers for specific use cases
    /// 
    /// # Examples
    /// 
    /// ```
    /// use serde::Deserialize;
    /// use std::io::Cursor;
    /// use mrf_rs::parser::MrfParser;
    /// 
    /// #[derive(Deserialize)]
    /// struct CustomHeader {
    ///     reporting_entity_name: String,
    ///     version: String,
    /// }
    /// 
    /// let json = r#"{"reporting_entity_name": "Test", "version": "1.0"}"#;
    /// let cursor = Cursor::new(json.as_bytes());
    /// 
    /// let header: CustomHeader = MrfParser::parse_generic(cursor)?;
    /// assert_eq!(header.reporting_entity_name, "Test");
    /// # Ok::<(), mrf_rs::parser::ParseError>(())
    /// ```
    /// 
    /// # Advanced Usage
    /// 
    /// This method can be combined with `serde_json::value::RawValue` for
    /// lazy parsing of large files, allowing you to skip over sections you
    /// don't need to process.
    pub fn parse_generic<T, R>(reader: R) -> ParseResult<T>
    where
        T: DeserializeOwned,
        R: Read,
    {
        let result = serde_json::from_reader(reader)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    
    #[test]
    fn test_parse_minimal_in_network() {
        let json = r#"{
            "reporting_entity_name": "Test Entity",
            "reporting_entity_type": "health insurance issuer",
            "in_network": [],
            "last_updated_on": "2024-01-01",
            "version": "1.0.0"
        }"#;
        
        let result = MrfParser::parse_str(json);
        assert!(result.is_ok());
        
        match result.unwrap() {
            MrfFile::InNetwork(file) => {
                assert_eq!(file.reporting_entity_name, "Test Entity");
                assert_eq!(file.reporting_entity_type, EntityType::HealthInsuranceIssuer);
                assert_eq!(file.version, "1.0.0");
            }
            _ => panic!("Expected InNetwork file type"),
        }
    }
    
    #[test]
    fn test_parse_table_of_contents() {
        let json = r#"{
            "reporting_entity_name": "Test Entity",
            "reporting_entity_type": "Third-Party Administrator",
            "reporting_structure": [],
            "version": "1.0.0"
        }"#;
        
        let result = MrfParser::parse_str(json);
        assert!(result.is_ok());
        
        match result.unwrap() {
            MrfFile::TableOfContents(file) => {
                assert_eq!(file.reporting_entity_name, "Test Entity");
                assert_eq!(file.reporting_entity_type, EntityType::ThirdPartyAdministrator);
                assert_eq!(file.version, Some("1.0.0".to_string()));
            }
            _ => panic!("Expected TableOfContents file type"),
        }
    }
    
    #[test]
    fn test_parse_allowed_amount() {
        let json = r#"{
            "reporting_entity_name": "Test Entity",
            "reporting_entity_type": "Third-Party Administrator",
            "out_of_network": [],
            "last_updated_on": "2024-01-01",
            "version": "1.0.0"
        }"#;
        
        let result = MrfParser::parse_str(json);
        assert!(result.is_ok());
        
        match result.unwrap() {
            MrfFile::AllowedAmount(file) => {
                assert_eq!(file.reporting_entity_name, "Test Entity");
                assert_eq!(file.reporting_entity_type, EntityType::ThirdPartyAdministrator);
                assert_eq!(file.version, "1.0.0");
            }
            _ => panic!("Expected AllowedAmount file type"),
        }
    }
    
    #[test]
    fn test_parse_provider_reference() {
        let json = r#"{
            "provider_groups": [],
            "version": "1.0.0"
        }"#;
        
        let result = MrfParser::parse_str(json);
        assert!(result.is_ok());
        
        match result.unwrap() {
            MrfFile::ProviderReference(file) => {
                assert_eq!(file.version, "1.0.0");
                assert_eq!(file.provider_groups.len(), 0);
            }
            _ => panic!("Expected ProviderReference file type"),
        }
    }
    
    #[test]
    fn test_parse_invalid_json() {
        let json = r#"{ invalid json }"#;
        let result = MrfParser::parse_str(json);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_from_reader() {
        let json = r#"{
            "reporting_entity_name": "Test Entity",
            "reporting_entity_type": "health insurance issuer",
            "in_network": [],
            "last_updated_on": "2024-01-01",
            "version": "1.0.0"
        }"#;
        
        let cursor = std::io::Cursor::new(json.as_bytes());
        let result = MrfParser::parse_reader(cursor);
        assert!(result.is_ok());
    }
} 