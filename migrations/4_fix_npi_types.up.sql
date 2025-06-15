-- Fix NPI column types to support full NPI range (up to 9,999,999,999)
-- NPIs are 10-digit numbers that can exceed INTEGER range

-- Update provider_groups table
ALTER TABLE mrf.provider_groups 
    ALTER COLUMN npi TYPE BIGINT[] USING npi::BIGINT[];

-- Update negotiated_rate_provider_groups table
ALTER TABLE mrf.negotiated_rate_provider_groups 
    ALTER COLUMN npi TYPE BIGINT[] USING npi::BIGINT[];

-- Update payment_providers table
ALTER TABLE mrf.payment_providers 
    ALTER COLUMN npi TYPE BIGINT[] USING npi::BIGINT[];

-- Add comment explaining the change
COMMENT ON COLUMN mrf.provider_groups.npi IS 'National Provider Identifier(s) - 10-digit numbers stored as BIGINT array';
COMMENT ON COLUMN mrf.negotiated_rate_provider_groups.npi IS 'National Provider Identifier(s) - 10-digit numbers stored as BIGINT array';
COMMENT ON COLUMN mrf.payment_providers.npi IS 'National Provider Identifier(s) - 10-digit numbers stored as BIGINT array'; 