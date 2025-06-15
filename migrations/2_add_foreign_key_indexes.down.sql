-- Drop all foreign key indexes created in the up migration

-- Drop indexes for taxonomy_code table
DROP INDEX IF EXISTS idx_taxonomy_code_npi;

-- Drop indexes for practice_location_record table
DROP INDEX IF EXISTS idx_practice_location_record_npi;
DROP INDEX IF EXISTS idx_practice_location_record_address_id;

-- Drop indexes for other_name_record table
DROP INDEX IF EXISTS idx_other_name_record_npi;

-- Drop indexes for other_identifier table
DROP INDEX IF EXISTS idx_other_identifier_npi;

-- Drop indexes for nppes_record table
DROP INDEX IF EXISTS idx_nppes_record_replacement_npi;
DROP INDEX IF EXISTS idx_nppes_record_provider_other_name_id;
DROP INDEX IF EXISTS idx_nppes_record_provider_name_id;
DROP INDEX IF EXISTS idx_nppes_record_practice_address_id;
DROP INDEX IF EXISTS idx_nppes_record_organization_name_id;
DROP INDEX IF EXISTS idx_nppes_record_mailing_address_id;
DROP INDEX IF EXISTS idx_nppes_record_authorized_official_id;

-- Drop indexes for endpoint_record table
DROP INDEX IF EXISTS idx_endpoint_record_npi;
DROP INDEX IF EXISTS idx_endpoint_record_affiliation_address_id; 