DROP TABLE IF EXISTS "onchain_document_signatures";
DROP TABLE IF EXISTS "onchain_documents";
DROP TABLE IF EXISTS "onchain_positions";
DROP TABLE IF EXISTS "onchain_divisions";
DROP TABLE IF EXISTS "onchain_officers";
CREATE TABLE "onchain_officers"(
	"id" BIGSERIAL PRIMARY KEY,
	"address" VARCHAR(255) NOT NULL UNIQUE,
	"name" VARCHAR(255) NOT NULL,
	"date_of_birth" VARCHAR(255) NOT NULL,
	"sex" VARCHAR(255) NOT NULL,
	"status" SMALLINT NOT NULL
);
CREATE TABLE "onchain_divisions"(
	"id" BIGSERIAL PRIMARY KEY,
	"onchain_id" VARCHAR(255) NOT NULL UNIQUE,
	"name" VARCHAR(255) NOT NULL,
	"supervisory_id" VARCHAR(255),
	"status" SMALLINT NOT NULL
);
CREATE TABLE "onchain_positions"(
	"officer_address" VARCHAR(255) NOT NULL,
	"division_onchain_id" VARCHAR(255) NOT NULL,
	"position_index" SMALLINT NOT NULL,
	"name" VARCHAR(255) NOT NULL,
	"role" SMALLINT NOT NULL,
	PRIMARY KEY(
		"officer_address",
		"division_onchain_id",
		"position_index"
	)
);
CREATE TABLE IF NOT EXISTS "onchain_documents" (
	"id" BIGSERIAL PRIMARY KEY,
	"document_content_hash" VARCHAR(255) NOT NULL UNIQUE,
	"number" VARCHAR(255) NOT NULL,
	"name" VARCHAR(255) NOT NULL,
	"doc_type" VARCHAR(255) NOT NULL,
	"division_id" VARCHAR(255) NOT NULL,
	"published_timestamp" TIMESTAMP NOT NULL,
	"publisher_address" VARCHAR(255) NOT NULL,
	"publisher_division_id" VARCHAR(255) NOT NULL,
	"publisher_position_index" SMALLINT NOT NULL
);
CREATE TABLE IF NOT EXISTS "onchain_document_signatures" (
	"document_content_hash" VARCHAR(255) NOT NULL,
	"signers_address" VARCHAR(255) NOT NULL,
	"division_onchain_id" VARCHAR(255) NOT NULL,
	"position_index" SMALLINT NOT NULL,
	PRIMARY KEY(
		"document_content_hash",
		"signers_address",
		"division_onchain_id",
		"position_index"
	)
);