-- Drop all tables in reverse order of creation to respect foreign key constraints

-- Drop indexes first (they will be dropped with tables, but being explicit)
DROP INDEX IF EXISTS mrf.idx_allowed_amounts_tin;
DROP INDEX IF EXISTS mrf.idx_allowed_amounts_rate;
DROP INDEX IF EXISTS mrf.idx_out_network_code;
DROP INDEX IF EXISTS mrf.idx_out_network_file;
DROP INDEX IF EXISTS mrf.idx_negotiated_prices_type;
DROP INDEX IF EXISTS mrf.idx_negotiated_prices_exp;
DROP INDEX IF EXISTS mrf.idx_negotiated_prices_detail;
DROP INDEX IF EXISTS mrf.idx_negotiated_rate_provider_refs;
DROP INDEX IF EXISTS mrf.idx_negotiated_rate_details;
DROP INDEX IF EXISTS mrf.idx_in_network_name;
DROP INDEX IF EXISTS mrf.idx_in_network_code;
DROP INDEX IF EXISTS mrf.idx_in_network_file;
DROP INDEX IF EXISTS mrf.idx_provider_groups_tin;
DROP INDEX IF EXISTS mrf.idx_provider_groups_npi;
DROP INDEX IF EXISTS mrf.idx_provider_groups_ref;
DROP INDEX IF EXISTS mrf.idx_provider_refs_group;
DROP INDEX IF EXISTS mrf.idx_provider_refs_file;
DROP INDEX IF EXISTS mrf.idx_mrf_files_plan;
DROP INDEX IF EXISTS mrf.idx_mrf_files_updated;
DROP INDEX IF EXISTS mrf.idx_mrf_files_entity;

-- Drop tables in reverse order
DROP TABLE IF EXISTS mrf.payment_providers CASCADE;
DROP TABLE IF EXISTS mrf.payments CASCADE;
DROP TABLE IF EXISTS mrf.allowed_amounts CASCADE;
DROP TABLE IF EXISTS mrf.out_of_network_rates CASCADE;
DROP TABLE IF EXISTS mrf.negotiated_prices CASCADE;
DROP TABLE IF EXISTS mrf.negotiated_rate_provider_groups CASCADE;
DROP TABLE IF EXISTS mrf.negotiated_rate_details CASCADE;
DROP TABLE IF EXISTS mrf.covered_services CASCADE;
DROP TABLE IF EXISTS mrf.bundled_codes CASCADE;
DROP TABLE IF EXISTS mrf.in_network_rates CASCADE;
DROP TABLE IF EXISTS mrf.provider_groups CASCADE;
DROP TABLE IF EXISTS mrf.provider_references CASCADE;
DROP TABLE IF EXISTS mrf.mrf_files CASCADE;

-- Drop custom types
DROP TYPE IF EXISTS mrf.negotiation_arrangement CASCADE;
DROP TYPE IF EXISTS mrf.market_type CASCADE;
DROP TYPE IF EXISTS mrf.plan_id_type CASCADE;
DROP TYPE IF EXISTS mrf.billing_class CASCADE;
DROP TYPE IF EXISTS mrf.negotiated_type CASCADE;
DROP TYPE IF EXISTS mrf.billing_code_type CASCADE;
DROP TYPE IF EXISTS mrf.entity_type CASCADE;

-- Drop schema
DROP SCHEMA IF EXISTS mrf CASCADE; 