INSERT INTO "divisions" ("onchain_id", "name", "supervisory_id", "status")
VALUES (
        $1,
        $2,
        (
            SELECT id
            FROM divisions d2
            WHERE d2.onchain_id = $3
        ),
        $4
    ) ON CONFLICT ("onchain_id") DO NOTHING;