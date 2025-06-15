-- Create mrf schema
CREATE SCHEMA IF NOT EXISTS mrf;

-- Set search path
SET search_path TO mrf, public;

-- Create custom types
CREATE TYPE mrf.entity_type AS ENUM (
    'group_health_plan',
    'health_insurance_issuer',
    'third_party_administrator',
    'healthcare_clearinghouse',
    'other'
);

CREATE TYPE mrf.billing_code_type AS ENUM (
    'CPT',
    'NDC',
    'HCPCS',
    'RC',
    'ICD',
    'MS-DRG',
    'R-DRG',
    'APC',
    'CDT',
    'CSTM-ALL',
    'OTHER'
);

CREATE TYPE mrf.negotiated_type AS ENUM (
    'negotiated',
    'derived',
    'fee',
    'percentage',
    'per_diem'
);

CREATE TYPE mrf.billing_class AS ENUM (
    'professional',
    'institutional',
    'professional-institutional'
);

CREATE TYPE mrf.plan_id_type AS ENUM (
    'ein',
    'hios'
);

CREATE TYPE mrf.market_type AS ENUM (
    'group',
    'individual'
);

CREATE TYPE mrf.negotiation_arrangement AS ENUM (
    'ffs',
    'bundle',
    'capitation'
);

-- Create tables

-- MRF files metadata
CREATE TABLE mrf.mrf_files (
    id BIGSERIAL PRIMARY KEY,
    reporting_entity_name VARCHAR(255) NOT NULL,
    reporting_entity_type mrf.entity_type NOT NULL,
    plan_name VARCHAR(255),
    plan_id_type mrf.plan_id_type,
    plan_id VARCHAR(100),
    plan_market_type mrf.market_type,
    last_updated_on DATE NOT NULL,
    version VARCHAR(50) NOT NULL,
    source_url TEXT,
    file_size_bytes BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    processed_at TIMESTAMPTZ DEFAULT NOW()
);

-- Provider references for deduplication
CREATE TABLE mrf.provider_references (
    id BIGSERIAL PRIMARY KEY,
    mrf_file_id BIGINT NOT NULL REFERENCES mrf.mrf_files(id) ON DELETE CASCADE,
    provider_group_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Provider groups
CREATE TABLE mrf.provider_groups (
    id BIGSERIAL PRIMARY KEY,
    provider_reference_id BIGINT NOT NULL REFERENCES mrf.provider_references(id) ON DELETE CASCADE,
    npi INTEGER[] NOT NULL,
    tin_type VARCHAR(10) NOT NULL,
    tin_value VARCHAR(20) NOT NULL
);

-- In-network rates
CREATE TABLE mrf.in_network_rates (
    id BIGSERIAL PRIMARY KEY,
    mrf_file_id BIGINT NOT NULL REFERENCES mrf.mrf_files(id) ON DELETE CASCADE,
    negotiation_arrangement mrf.negotiation_arrangement NOT NULL,
    name VARCHAR(500) NOT NULL,
    billing_code_type mrf.billing_code_type NOT NULL,
    billing_code_type_version VARCHAR(50) NOT NULL,
    billing_code VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Bundled codes for bundle arrangements
CREATE TABLE mrf.bundled_codes (
    id BIGSERIAL PRIMARY KEY,
    in_network_rate_id BIGINT NOT NULL REFERENCES mrf.in_network_rates(id) ON DELETE CASCADE,
    billing_code_type mrf.billing_code_type NOT NULL,
    billing_code_type_version VARCHAR(50) NOT NULL,
    billing_code VARCHAR(50) NOT NULL,
    description TEXT
);

-- Covered services for capitation arrangements
CREATE TABLE mrf.covered_services (
    id BIGSERIAL PRIMARY KEY,
    in_network_rate_id BIGINT NOT NULL REFERENCES mrf.in_network_rates(id) ON DELETE CASCADE,
    billing_code_type mrf.billing_code_type NOT NULL,
    billing_code_type_version VARCHAR(50) NOT NULL,
    billing_code VARCHAR(50) NOT NULL,
    description TEXT
);

-- Negotiated rate details
CREATE TABLE mrf.negotiated_rate_details (
    id BIGSERIAL PRIMARY KEY,
    in_network_rate_id BIGINT NOT NULL REFERENCES mrf.in_network_rates(id) ON DELETE CASCADE,
    provider_references INTEGER[]
);

-- Provider groups for negotiated rate details
CREATE TABLE mrf.negotiated_rate_provider_groups (
    id BIGSERIAL PRIMARY KEY,
    negotiated_rate_detail_id BIGINT NOT NULL REFERENCES mrf.negotiated_rate_details(id) ON DELETE CASCADE,
    npi INTEGER[] NOT NULL,
    tin_type VARCHAR(10) NOT NULL,
    tin_value VARCHAR(20) NOT NULL
);

-- Negotiated prices
CREATE TABLE mrf.negotiated_prices (
    id BIGSERIAL PRIMARY KEY,
    negotiated_rate_detail_id BIGINT NOT NULL REFERENCES mrf.negotiated_rate_details(id) ON DELETE CASCADE,
    negotiated_type mrf.negotiated_type NOT NULL,
    negotiated_rate DECIMAL(15,2) NOT NULL,
    expiration_date DATE NOT NULL,
    service_code VARCHAR(10)[],
    billing_class mrf.billing_class NOT NULL,
    billing_code_modifier VARCHAR(10)[],
    additional_information TEXT
);

-- Out-of-network rates
CREATE TABLE mrf.out_of_network_rates (
    id BIGSERIAL PRIMARY KEY,
    mrf_file_id BIGINT NOT NULL REFERENCES mrf.mrf_files(id) ON DELETE CASCADE,
    name VARCHAR(500) NOT NULL,
    billing_code_type mrf.billing_code_type NOT NULL,
    billing_code_type_version VARCHAR(50) NOT NULL,
    billing_code VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Allowed amounts for out-of-network
CREATE TABLE mrf.allowed_amounts (
    id BIGSERIAL PRIMARY KEY,
    out_of_network_rate_id BIGINT NOT NULL REFERENCES mrf.out_of_network_rates(id) ON DELETE CASCADE,
    tin_type VARCHAR(10) NOT NULL,
    tin_value VARCHAR(20) NOT NULL,
    service_code VARCHAR(10)[] NOT NULL,
    billing_class mrf.billing_class NOT NULL
);

-- Payments for allowed amounts
CREATE TABLE mrf.payments (
    id BIGSERIAL PRIMARY KEY,
    allowed_amount_id BIGINT NOT NULL REFERENCES mrf.allowed_amounts(id) ON DELETE CASCADE,
    allowed_amount DECIMAL(15,2) NOT NULL,
    billing_code_modifier VARCHAR(10)[]
);

-- Providers for payments
CREATE TABLE mrf.payment_providers (
    id BIGSERIAL PRIMARY KEY,
    payment_id BIGINT NOT NULL REFERENCES mrf.payments(id) ON DELETE CASCADE,
    npi INTEGER[] NOT NULL,
    billed_charge DECIMAL(15,2) NOT NULL
);

-- Create indexes for performance

-- MRF files indexes
CREATE INDEX idx_mrf_files_entity ON mrf.mrf_files(reporting_entity_name);
CREATE INDEX idx_mrf_files_updated ON mrf.mrf_files(last_updated_on);
CREATE INDEX idx_mrf_files_plan ON mrf.mrf_files(plan_id) WHERE plan_id IS NOT NULL;

-- Provider references indexes
CREATE INDEX idx_provider_refs_file ON mrf.provider_references(mrf_file_id);
CREATE INDEX idx_provider_refs_group ON mrf.provider_references(provider_group_id);

-- Provider groups indexes
CREATE INDEX idx_provider_groups_ref ON mrf.provider_groups(provider_reference_id);
CREATE INDEX idx_provider_groups_npi ON mrf.provider_groups USING GIN(npi);
CREATE INDEX idx_provider_groups_tin ON mrf.provider_groups(tin_value);

-- In-network rates indexes
CREATE INDEX idx_in_network_file ON mrf.in_network_rates(mrf_file_id);
CREATE INDEX idx_in_network_code ON mrf.in_network_rates(billing_code_type, billing_code);
CREATE INDEX idx_in_network_name ON mrf.in_network_rates(name);

-- Negotiated rate details indexes
CREATE INDEX idx_negotiated_rate_details ON mrf.negotiated_rate_details(in_network_rate_id);
CREATE INDEX idx_negotiated_rate_provider_refs ON mrf.negotiated_rate_details USING GIN(provider_references) WHERE provider_references IS NOT NULL;

-- Negotiated prices indexes
CREATE INDEX idx_negotiated_prices_detail ON mrf.negotiated_prices(negotiated_rate_detail_id);
CREATE INDEX idx_negotiated_prices_exp ON mrf.negotiated_prices(expiration_date);
CREATE INDEX idx_negotiated_prices_type ON mrf.negotiated_prices(negotiated_type);

-- Out-of-network rates indexes
CREATE INDEX idx_out_network_file ON mrf.out_of_network_rates(mrf_file_id);
CREATE INDEX idx_out_network_code ON mrf.out_of_network_rates(billing_code_type, billing_code);

-- Allowed amounts indexes
CREATE INDEX idx_allowed_amounts_rate ON mrf.allowed_amounts(out_of_network_rate_id);
CREATE INDEX idx_allowed_amounts_tin ON mrf.allowed_amounts(tin_value);

-- Comments for documentation
COMMENT ON SCHEMA mrf IS 'Schema for Machine Readable Files (MRF) healthcare price transparency data';
COMMENT ON TABLE mrf.mrf_files IS 'Metadata for processed MRF files';
COMMENT ON TABLE mrf.provider_references IS 'Provider reference lookup for deduplication';
COMMENT ON TABLE mrf.in_network_rates IS 'In-network negotiated rates for services';
COMMENT ON TABLE mrf.out_of_network_rates IS 'Out-of-network allowed amounts for services';
COMMENT ON TABLE mrf.negotiated_prices IS 'Specific negotiated prices with expiration dates'; 