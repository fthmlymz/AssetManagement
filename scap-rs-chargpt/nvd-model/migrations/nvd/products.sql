create table nvd.products
(
    id          binary(16)                                    not null comment '产品ID'
        primary key,
    vendor_id   binary(16)                                    not null comment '提供商外键',
    official    tinyint(4) unsigned default 0                 not null comment '是否为官方数据',
    part        char                default '*'               not null comment '硬件设备 h,操作系统 o,应用程序 a',
    name        varchar(128)                                  not null comment '产品名字',
    description text                                          not null comment '产品描述',
    meta        json                                          not null comment '元数据',
    created_at  timestamp           default CURRENT_TIMESTAMP not null comment '创建时间',
    updated_at  timestamp           default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '最后更新时间',
    constraint id_UNIQUE
        unique (id),
    constraint name_UNIQUE
        unique (vendor_id, name),
    constraint product_vendor
        foreign key (vendor_id) references nvd.vendors (id)
            on delete cascade
)
    comment '产品表';

create index vendor_idx
    on nvd.products (vendor_id);

postgresql için
create table nvd.products (
    id          uuid                        not null primary key, -- Product ID
    vendor_id   uuid                        not null,             -- Foreign key to vendor
    official    smallint default 0          not null,             -- Whether it's official data
    part        char(1) default '*'         not null,             -- h: hardware, o: OS, a: application
    name        varchar(128)                not null,             -- Product name
    description text                        not null,             -- Product description
    meta        jsonb                       not null,             -- Metadata
    created_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Created time
    updated_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Last updated time
    constraint id_unique unique (id),
    constraint name_unique unique (vendor_id, name),
    constraint product_vendor foreign key (vendor_id)
        references nvd.vendors (id)
        on delete cascade
);
-- Table comment: Product table

create index vendor_idx on nvd.products (vendor_id);
