-- Your SQL goes here

-- vendors tablosu
CREATE TABLE vendors (
    id BYTEA PRIMARY KEY,
    official SMALLINT NOT NULL,
    name VARCHAR(128) NOT NULL,
    description TEXT,
    meta JSONB,
    updated_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL
);

-- products tablosu
CREATE TABLE products (
    id BYTEA PRIMARY KEY,
    vendor_id BYTEA NOT NULL REFERENCES vendors(id),
    official SMALLINT NOT NULL,
    part CHAR(1) NOT NULL,
    name VARCHAR(128) NOT NULL,
    description TEXT,
    meta JSONB,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- cves tablosu
CREATE TABLE cves (
    id VARCHAR(32) PRIMARY KEY,
    year INTEGER NOT NULL,
    assigner VARCHAR(64) NOT NULL,
    description JSONB NOT NULL,
    translated SMALLINT NOT NULL DEFAULT 0,
    severity VARCHAR(32),
    metrics JSONB,
    weaknesses JSONB,
    configurations JSONB,
    references JSONB,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- cwes tablosu
CREATE TABLE cwes (
    id INTEGER PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    description TEXT,
    name_zh VARCHAR(256),
    description_zh TEXT,
    status VARCHAR(32),
    remediation TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- knowledge_base tablosu
CREATE TABLE knowledge_base (
    id BYTEA PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    types VARCHAR(32) NOT NULL,
    source VARCHAR(32) NOT NULL,
    verified SMALLINT NOT NULL,
    description TEXT,
    path VARCHAR(512) NOT NULL,
    meta JSONB,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- cve_product ilişki tablosu
CREATE TABLE cve_product (
    cve_id VARCHAR(32) NOT NULL REFERENCES cves(id),
    product_id BYTEA NOT NULL REFERENCES products(id),
    PRIMARY KEY (cve_id, product_id)
);

-- cve_knowledge_base ilişki tablosu
CREATE TABLE cve_knowledge_base (
    cve_id VARCHAR(32) NOT NULL REFERENCES cves(id),
    knowledge_base_id BYTEA NOT NULL REFERENCES knowledge_base(id),
    PRIMARY KEY (cve_id, knowledge_base_id)
);
