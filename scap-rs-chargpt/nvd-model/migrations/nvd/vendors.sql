create table nvd.vendors
(
    id          binary(16)                                    not null comment '供应商ID'
        primary key,
    official    tinyint(4) unsigned default 0                 not null comment '是否为官方数据',
    name        varchar(128)                                  not null comment '供应商名字',
    description text                                          not null comment '供应商描述',
    meta        json                                          not null comment '元数据',
    updated_at  timestamp           default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '最后更新时间',
    created_at  timestamp           default CURRENT_TIMESTAMP not null comment '创建时间',
    constraint id_UNIQUE
        unique (id),
    constraint name_UNIQUE
        unique (name)
)
    comment '供应商表';



postgresql için
create table nvd.vendors (
    id          uuid                        not null primary key, -- Vendor ID
    official    smallint default 0          not null,             -- Whether it's official data
    name        varchar(128)                not null,             -- Vendor name
    description text                        not null,             -- Vendor description
    meta        jsonb                       not null,             -- Metadata
    updated_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Last updated time
    created_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Created time
    constraint id_unique unique (id),
    constraint name_unique unique (name)
);
-- Table comment: Vendor table
