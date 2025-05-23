// @generated automatically by Diesel CLI.

diesel::table! {
    cve_product (cve_id, product_id) {
        #[max_length = 16]
        cve_id -> Varchar,
        #[max_length = 16]
        product_id -> Bytea,
    }
}

diesel::table! {
    cve_knowledge_base (cve_id, knowledge_base_id) {
        #[max_length = 16]
        cve_id -> Varchar,
        #[max_length = 16]
        knowledge_base_id -> Bytea,
    }
}

diesel::table! {
    cves (id) {
        #[max_length = 32]
        id -> Varchar,
        year -> Integer,
        #[max_length = 64]
        assigner -> Varchar,
        description -> Jsonb,
        translated -> Smallint,
        #[max_length = 32]
        severity -> Varchar,
        metrics -> Jsonb,
        weaknesses -> Jsonb,
        configurations -> Jsonb,
        references -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cwes (id) {
        id -> Integer,
        #[max_length = 256]
        name -> Varchar,
        description -> Text,
        #[max_length = 256]
        name_zh -> Varchar,
        description_zh -> Text,
        #[max_length = 32]
        status -> Varchar,
        remediation -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        #[max_length = 16]
        id -> Bytea,
        #[max_length = 16]
        vendor_id -> Bytea,
        official -> Smallint,
        #[max_length = 1]
        part -> Char,
        #[max_length = 128]
        name -> Varchar,
        description -> Text,
        meta -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    vendors (id) {
        #[max_length = 16]
        id -> Bytea,
        official -> Smallint,
        #[max_length = 128]
        name -> Varchar,
        description -> Text,
        meta -> Jsonb,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    knowledge_base (id) {
        #[max_length = 16]
        id -> Bytea,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 32]
        types -> Varchar,
        #[max_length = 32]
        source -> Varchar,
        verified -> Smallint,
        description -> Text,
        #[max_length = 512]
        path -> Varchar,
        meta -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(cve_product -> cves (cve_id));
diesel::joinable!(cve_product -> products (product_id));
diesel::joinable!(products -> vendors (vendor_id));
diesel::joinable!(cve_knowledge_base -> cves (cve_id));
diesel::joinable!(cve_knowledge_base -> knowledge_base (knowledge_base_id));

diesel::allow_tables_to_appear_in_same_query!(
  cve_product,
  cves,
  cwes,
  products,
  vendors,
  cve_knowledge_base,
  knowledge_base,
);
