create table nvd.cwes
(
    id          int                                 not null comment 'CWE ID'
        primary key,
    name        varchar(256)                        not null comment 'CWE 名称',
    description text                                not null comment 'CWE描述',
    created_at  timestamp default CURRENT_TIMESTAMP not null comment '创建时间',
    updated_at  timestamp default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '最后更新时间',
    constraint id_UNIQUE
        unique (id),
    constraint name_UNIQUE
        unique (name)
)
    comment '弱点枚举';


postgresql için
CREATE TABLE nvd.cwes (
    id          integer                             not null primary key, -- CWE ID
    name        varchar(256)                        not null,             -- CWE name
    description text                                not null,             -- CWE description
    created_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Created time
    updated_at  timestamp without time zone default CURRENT_TIMESTAMP not null, -- Last updated time
    constraint cwes_name_unique unique (name)
);
