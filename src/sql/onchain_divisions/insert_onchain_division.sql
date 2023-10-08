INSERT INTO "onchain_divisions" ("onchain_id", "name", "supervisory_id", "status")
VALUES (
        $1,
        $2,
        $3,
        $4
    ) ON CONFLICT ("onchain_id") DO NOTHING;