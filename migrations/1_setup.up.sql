-- ENUMS (same as before)
CREATE TYPE entity_type AS ENUM ('Individual', 'Organization');
CREATE TYPE subpart_code AS ENUM ('NotAnswered', 'Yes', 'No');
CREATE TYPE group_taxonomy_code AS ENUM ('MultiSpecialtyGroup', 'SingleSpecialtyGroup');
CREATE TYPE state_code AS ENUM (
  'AK','AL','AR','AS','AZ','CA','CO','CT','DC','DE','FL','FM','GA','GU','HI','IA','ID','IL','IN','KS','KY','LA','MA','MD','ME','MH','MI','MN','MO','MP','MS','MT','NC','ND','NE','NH','NJ','NM','NV','NY','OH','OK','OR','PA','PR','PW','RI','SC','SD','TN','TX','UT','VA','VI','VT','WA','WI','WV','WY','ZZ'
);
CREATE TYPE deactivation_reason_code AS ENUM ('Death', 'Disbandment', 'Fraud', 'Other', 'Undisclosed');
CREATE TYPE sole_proprietor_code AS ENUM ('NotAnswered', 'Yes', 'No');
CREATE TYPE primary_taxonomy_switch AS ENUM ('NotAnswered', 'Yes', 'No');
CREATE TYPE other_provider_identifier_issuer_code AS ENUM ('Other', 'Medicaid');
CREATE TYPE other_provider_name_type_code AS ENUM ('FormerName', 'ProfessionalName', 'DoingBusinessAs', 'FormerLegalBusinessName', 'OtherName');
CREATE TYPE sex_code AS ENUM ('Male', 'Female', 'Undisclosed');
CREATE TYPE name_prefix_code AS ENUM ('Ms', 'Mr', 'Miss', 'Mrs', 'Dr', 'Prof');
CREATE TYPE name_suffix_code AS ENUM ('Jr','Sr','I','II','III','IV','V','VI','VII','VIII','IX','X');

-- 1. Lookup/child tables
CREATE TABLE provider_name (
    id BIGSERIAL PRIMARY KEY,
    prefix name_prefix_code,
    first text,
    middle text,
    last text,
    suffix name_suffix_code,
    credential text
);

CREATE TABLE organization_name (
    id BIGSERIAL PRIMARY KEY,
    legal_business_name text,
    other_name text,
    other_name_type other_provider_name_type_code
);

CREATE TABLE address (
    id BIGSERIAL PRIMARY KEY,
    line_1 text,
    line_2 text,
    city text,
    postal_code text,
    telephone text,
    fax text,
    state state_code,
    country text
);

CREATE TABLE authorized_official (
    id BIGSERIAL PRIMARY KEY,
    prefix name_prefix_code,
    first_name text,
    middle_name text,
    last_name text,
    suffix name_suffix_code,
    credential text,
    title text,
    telephone text
);

-- 2. Main table
CREATE TABLE nppes_record (
    npi text PRIMARY KEY,
    entity_type entity_type,
    replacement_npi text,
    ein text,
    provider_name_id BIGINT REFERENCES provider_name(id),
    provider_other_name_id BIGINT REFERENCES provider_name(id),
    provider_other_name_type other_provider_name_type_code,
    organization_name_id BIGINT REFERENCES organization_name(id),
    mailing_address_id BIGINT REFERENCES address(id),
    practice_address_id BIGINT REFERENCES address(id),
    enumeration_date DATE,
    last_update_date DATE,
    deactivation_date DATE,
    reactivation_date DATE,
    certification_date DATE,
    deactivation_reason deactivation_reason_code,
    provider_gender sex_code,
    authorized_official_id BIGINT REFERENCES authorized_official(id),
    sole_proprietor sole_proprietor_code,
    organization_subpart subpart_code,
    parent_organization_lbn text,
    parent_organization_tin text,
    FOREIGN KEY (replacement_npi) REFERENCES nppes_record(npi)
);

-- 3. Tables that reference nppes_record
CREATE TABLE taxonomy_code (
    id BIGSERIAL PRIMARY KEY,
    npi text REFERENCES nppes_record(npi),
    code text,
    license_number text,
    license_state state_code,
    is_primary BOOLEAN,
    taxonomy_group text,
    group_taxonomy_code group_taxonomy_code,
    primary_switch primary_taxonomy_switch
);

CREATE TABLE other_identifier (
    id BIGSERIAL PRIMARY KEY,
    npi text REFERENCES nppes_record(npi),
    identifier text,
    type_code text,
    issuer other_provider_identifier_issuer_code,
    state state_code
);

CREATE TABLE practice_location_record (
    id BIGSERIAL PRIMARY KEY,
    npi text REFERENCES nppes_record(npi),
    address_id BIGINT REFERENCES address(id),
    telephone_extension text
);

CREATE TABLE endpoint_record (
    id BIGSERIAL PRIMARY KEY,
    npi text REFERENCES nppes_record(npi),
    endpoint_type text,
    endpoint_type_description text,
    endpoint text,
    affiliation BOOLEAN,
    endpoint_description text,
    affiliation_legal_business_name text,
    use_code text,
    use_description text,
    other_use_description text,
    content_type text,
    content_description text,
    other_content_description text,
    affiliation_address_id BIGINT REFERENCES address(id)
);

CREATE TABLE other_name_record (
    id BIGSERIAL PRIMARY KEY,
    npi text REFERENCES nppes_record(npi),
    provider_other_organization_name text,
    provider_other_organization_name_type_code text
);

CREATE TABLE taxonomy_reference (
    code text PRIMARY KEY,
    grouping text,
    classification text,
    specialization text,
    definition text,
    notes text,
    display_name text,
    section text
);