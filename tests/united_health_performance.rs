use mrf_rs::sources::united_health::{UnitedHealthSource, UnitedHealthConfig};
use mrf_rs::sources::MrfSource;
use std::time::Instant;
use test_log::test;

#[test(tokio::test)]
async fn test_fetch_all_index_files() {
    
    let source = UnitedHealthSource::new().unwrap();
    
    let start = Instant::now();
    let index_files = source.fetch_all_index_files().await.unwrap();
    let duration = start.elapsed();
    
    println!("\n=== Fetch All Index Files ===");
    println!("Total index files: {}", index_files.len());
    println!("Time taken: {:.2?}", duration);
    println!("Files per second: {:.2}", index_files.len() as f64 / duration.as_secs_f64());
}

#[test(tokio::test)]
async fn test_api_response_parsing_speed() {
    let client = reqwest::Client::new();
    
    // Fetch the API response
    let fetch_start = Instant::now();
    let response = client
        .get("https://transparency-in-coverage.uhc.com/api/v1/uhc/blobs")
        .send()
        .await
        .unwrap();
    let content = response.text().await.unwrap();
    let fetch_duration = fetch_start.elapsed();
    
    // Measure parsing speed
    let parse_start = Instant::now();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    let parse_duration = parse_start.elapsed();
    
    let blob_count = parsed["blobs"].as_array().map(|a| a.len()).unwrap_or(0);
    
    println!("\n=== API Response Parsing ===");
    println!("Response size: {} bytes", content.len());
    println!("Blob count: {}", blob_count);
    println!("Fetch time: {:.2?}", fetch_duration);
    println!("Parse time: {:.2?}", parse_duration);
    println!("Parse throughput: {:.2} MB/s", 
             content.len() as f64 / 1_048_576.0 / parse_duration.as_secs_f64());
}

#[test(tokio::test)]
async fn test_single_index_file_processing() {
    let client = reqwest::Client::new();
    
    // Get a sample index file URL
    let response = client
        .get("https://transparency-in-coverage.uhc.com/api/v1/uhc/blobs")
        .send()
        .await
        .unwrap();
    
    let content = response.text().await.unwrap();
    let api_response: serde_json::Value = serde_json::from_str(&content).unwrap();
    
    let sample_url = api_response["blobs"][0]["downloadUrl"]
        .as_str()
        .unwrap()
        .to_string();
    
    println!("\n=== Single Index File Processing ===");
    println!("Index file URL: {}", sample_url);
    
    // Fetch the index file
    let fetch_start = Instant::now();
    let response = client.get(&sample_url).send().await.unwrap();
    let content = response.bytes().await.unwrap();
    let fetch_duration = fetch_start.elapsed();
    
    // Parse the index file
    let parse_start = Instant::now();
    let index: serde_json::Value = serde_json::from_slice(&content).unwrap();
    let parse_duration = parse_start.elapsed();
    
    let reporting_structures = index["reporting_structure"].as_array().map(|a| a.len()).unwrap_or(0);
    
    println!("File size: {} bytes", content.len());
    println!("Reporting structures: {}", reporting_structures);
    println!("Fetch time: {:.2?}", fetch_duration);
    println!("Parse time: {:.2?}", parse_duration);
    println!("Parse throughput: {:.2} MB/s", 
             content.len() as f64 / 1_048_576.0 / parse_duration.as_secs_f64());
}

#[test(tokio::test(flavor = "multi_thread", worker_threads = 100))]
async fn test_full_discovery_performance() {
    // Use default configuration (no concurrency limits)
    let source = UnitedHealthSource::new().unwrap();
    
    println!("\n=== Full Discovery Performance ===");
    
    let start = Instant::now();
    let files = source.discover_files().await.unwrap();
    let duration = start.elapsed();
    
    // Count file types
    let in_network_count = files.iter()
        .filter(|f| matches!(f.file_type, mrf_rs::sources::MrfFileType::InNetwork))
        .count();
    let allowed_amount_count = files.iter()
        .filter(|f| matches!(f.file_type, mrf_rs::sources::MrfFileType::AllowedAmount))
        .count();
    
    println!("Total MRF files discovered: {}", files.len());
    println!("  In-Network files: {}", in_network_count);
    println!("  Allowed Amount files: {}", allowed_amount_count);
    println!("Total time: {:.2?}", duration);
    println!("Files per second: {:.2}", files.len() as f64 / duration.as_secs_f64());
    println!("Average time per file: {:.2?}", duration / files.len() as u32);
} 