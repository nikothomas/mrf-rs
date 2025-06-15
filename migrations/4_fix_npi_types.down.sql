-- Revert NPI column types back to INTEGER[]
-- WARNING: This may fail if any NPIs exceed INTEGER range (2,147,483,647)

-- Revert provider_groups table
ALTER TABLE mrf.provider_groups 
    ALTER COLUMN npi TYPE INTEGER[] USING npi::INTEGER[];

-- Revert negotiated_rate_provider_groups table
ALTER TABLE mrf.negotiated_rate_provider_groups 
    ALTER COLUMN npi TYPE INTEGER[] USING npi::INTEGER[];

-- Revert payment_providers table
ALTER TABLE mrf.payment_providers 
    ALTER COLUMN npi TYPE INTEGER[] USING npi::INTEGER[]; 