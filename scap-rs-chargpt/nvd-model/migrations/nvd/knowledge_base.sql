create table nvd.knowledge_base
(
    id          binary(16)                          not null comment '知识库表ID'
        primary key,
    name        varchar(32)                         not null comment '关联cve的名称',
    description text                                not null comment '知识库描述',
    source      varchar(32)                         not null comment '来源',
    links       varchar(512)                        not null comment '路径',
    meta        json                                not null comment '元数据',
    created_at  timestamp default CURRENT_TIMESTAMP not null comment '创建时间',
    updated_at  timestamp default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '最后更新时间',
    constraint id_UNIQUE
        unique (id),
    constraint knowledge_base_UNIQUE
        unique (name, source, links),
    constraint kb_cve
        foreign key (name) references nvd.cves (id)
)
    comment '知识库表';


postgresql için
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
