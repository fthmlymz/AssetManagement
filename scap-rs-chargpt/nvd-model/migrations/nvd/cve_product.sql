create table nvd.cve_product
(
    cve_id     varchar(16) not null comment 'CVE编号',
    product_id binary(16)  not null comment '产品ID',
    primary key (cve_id, product_id),
    constraint cve_id
        foreign key (cve_id) references nvd.cves (id),
    constraint product_id
        foreign key (product_id) references nvd.products (id)
)
    comment 'cve_match表';

create index product_id_idx
    on nvd.cve_product (product_id);


postgresql için
create table nvd.cve_product (
    cve_id     varchar(16) not null,  -- CVE number
    product_id uuid        not null,  -- Product ID
    primary key (cve_id, product_id),
    constraint fk_cve_id
        foreign key (cve_id) references nvd.cves (id),
    constraint fk_product_id
        foreign key (product_id) references nvd.products (id)
);
-- Table comment: CVE to Product match table

create index product_id_idx on nvd.cve_product (product_id);

