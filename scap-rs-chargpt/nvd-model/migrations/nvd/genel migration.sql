-- Önce nvd şemasını oluştur
CREATE SCHEMA IF NOT EXISTS nvd;

-- CVE tablosu
CREATE TABLE nvd.cves (
    id             varchar(32)                         not null primary key,  -- CVE number
    year           integer    default 0                 not null,            -- CVE year
    assigner       varchar(64)                         not null,             -- Assigner
    description    jsonb                               not null,             -- Description
    severity       varchar(32)                         not null,             -- Severity level
    metrics        jsonb                               not null,             -- CVSS metrics
    weaknesses     jsonb                               not null,             -- CWE references
    configurations jsonb                               not null,             -- CPE matches
    "references"   jsonb                               not null,             -- References (escaped with quotes)
    created_at     timestamp without time zone default CURRENT_TIMESTAMP not null, -- Created time
    updated_at     timestamp without time zone default CURRENT_TIMESTAMP not null, -- Last updated time
    constraint id_unique unique (id)
);


-- Vendor tablosu
CREATE TABLE nvd.vendors (
    id          uuid                        not null primary key, -- Vendor ID
    official    smallint default 0          not null,             -- Whether it's official data
    name        varchar(128)                not null,             -- Vendor name
    description text                        not null,             -- Vendor description
    meta        jsonb                       not null,             -- Metadata
    updated_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Last updated time
    created_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Created time
    constraint name_unique unique (name)
);



-- Product tablosu
CREATE TABLE nvd.products (
    id          uuid                        not null primary key, -- Product ID
    vendor_id   uuid                        not null,             -- Foreign key to vendor
    official    smallint default 0          not null,             -- Whether it's official data
    part        char(1) default '*'         not null,             -- h: hardware, o: OS, a: application
    name        varchar(128)                not null,             -- Product name
    description text                        not null,             -- Product description
    meta        jsonb                       not null,             -- Metadata
    created_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Created time
    updated_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Last updated time
    --constraint name_unique unique (vendor_id, name),
    constraint product_vendor foreign key (vendor_id)
        references nvd.vendors (id)
        on delete cascade
);




-----------------------------------------------------------------
-- References tablosu
/*CREATE TABLE nvd.references (
    id          uuid                        not null primary key, -- Reference ID
    cve_id      varchar(32)                not null,             -- CVE number
    url         varchar(512)               not null,             -- Reference URL
    name        varchar(256)               not null,             -- Reference name
    tags        jsonb                       not null,             -- Reference tags
    source      varchar(64)                not null,             -- Reference source
    created_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Created time
    updated_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Last updated time
    constraint id_unique unique (id),
    constraint fk_cve_id
        foreign key (cve_id) references nvd.cves (id)
        on delete cascade
);

-- İndeksler
CREATE INDEX references_cve_id_idx ON nvd.references (cve_id);
CREATE INDEX references_source_idx ON nvd.references (source);*/
-----------------------------------------------------------------

-- CVE-Product ilişki tablosu
CREATE TABLE nvd.cve_product (
    cve_id     varchar(16) not null,  -- CVE number
    product_id uuid        not null,  -- Product ID
    primary key (cve_id, product_id),
    constraint fk_cve_id
        foreign key (cve_id) references nvd.cves (id),
    constraint fk_product_id
        foreign key (product_id) references nvd.products (id)
);


CREATE TABLE nvd.cwes (
    id          integer                             not null primary key, -- CWE ID
    name        varchar(256)                        not null,             -- CWE name
    description text                                not null,             -- CWE description
    created_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Created time
    updated_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Last updated time
    constraint cwes_name_unique unique (name)
);


CREATE TABLE nvd.exploits (
    id          uuid                                not null primary key, -- Exploit table ID
    name        varchar(128)                        not null,             -- Exploit name
    description text                                not null,             -- Exploit description
    source      varchar(32)                         not null,             -- Source
    path        varchar(512)                        not null,             -- File path
    meta        jsonb                               not null,             -- Metadata
    verified    smallint                            not null,             -- Whether it's verified (0/1)
    created_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Created time
    updated_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Last updated time
    constraint exploits_unique unique (name, source, path)
);



CREATE TABLE nvd.knowledge_base (
    id          uuid                                not null primary key, -- Knowledge base ID
    name        varchar(32)                         not null,             -- Associated CVE ID
    description text                                not null,             -- Knowledge base description
    source      varchar(32)                         not null,             -- Source
    links       varchar(512)                        not null,             -- Link path
    meta        jsonb                               not null,             -- Metadata
    created_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Created time
    updated_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Last updated time
    constraint knowledge_base_unique unique (name, source, links),
    constraint kb_cve foreign key (name) references nvd.cves (id)
);






-- İndeksler
CREATE INDEX year_idx ON nvd.cves (year);
CREATE INDEX vendor_idx ON nvd.products (vendor_id);
CREATE INDEX product_id_idx ON nvd.cve_product (product_id);