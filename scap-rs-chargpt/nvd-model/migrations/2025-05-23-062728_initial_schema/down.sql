-- This file should undo anything in `up.sql`

-- İlişki tablolarını önce siliyoruz
DROP TABLE IF EXISTS cve_knowledge_base;
DROP TABLE IF EXISTS cve_product;

-- Ana tabloları siliyoruz
DROP TABLE IF EXISTS knowledge_base;
DROP TABLE IF EXISTS cwes;
DROP TABLE IF EXISTS cves;
DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS vendors;
