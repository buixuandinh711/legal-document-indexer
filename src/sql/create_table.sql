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
CREATE TABLE "onchain_documents" (
	"id" BIGSERIAL PRIMARY KEY,
	"hash" VARCHAR(255) NOT NULL UNIQUE,
	"division_onchain_id" VARCHAR(255) NOT NULL,
	"officer_address" VARCHAR(255) NOT NULL,
	"position_index" SMALLINT NOT NULL,
	"signer_onchain_id" VARCHAR(255) [] NOT NULL
);