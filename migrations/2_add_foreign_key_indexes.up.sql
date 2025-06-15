-- Add indexes for all unindexed foreign keys to improve query performance

-- Indexes for endpoint_record table
CREATE INDEX idx_endpoint_record_affiliation_address_id ON endpoint_record(affiliation_address_id);
CREATE INDEX idx_endpoint_record_npi ON endpoint_record(npi);

-- Indexes for nppes_record table
CREATE INDEX idx_nppes_record_authorized_official_id ON nppes_record(authorized_official_id);
CREATE INDEX idx_nppes_record_mailing_address_id ON nppes_record(mailing_address_id);
CREATE INDEX idx_nppes_record_organization_name_id ON nppes_record(organization_name_id);
CREATE INDEX idx_nppes_record_practice_address_id ON nppes_record(practice_address_id);
CREATE INDEX idx_nppes_record_provider_name_id ON nppes_record(provider_name_id);
CREATE INDEX idx_nppes_record_provider_other_name_id ON nppes_record(provider_other_name_id);
CREATE INDEX idx_nppes_record_replacement_npi ON nppes_record(replacement_npi);

-- Indexes for other_identifier table
CREATE INDEX idx_other_identifier_npi ON other_identifier(npi);

-- Indexes for other_name_record table
CREATE INDEX idx_other_name_record_npi ON other_name_record(npi);

-- Indexes for practice_location_record table
CREATE INDEX idx_practice_location_record_address_id ON practice_location_record(address_id);
CREATE INDEX idx_practice_location_record_npi ON practice_location_record(npi);

-- Indexes for taxonomy_code table
CREATE INDEX idx_taxonomy_code_npi ON taxonomy_code(npi); 