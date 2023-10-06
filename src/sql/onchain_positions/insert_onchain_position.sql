INSERT INTO "positions" (
        "onchain_officer_id",
        "division_id",
        "position_index",
        "name",
        "role"
    )
VALUES (
        (
            SELECT "id"
            FROM "onchain_officers" of
            WHERE of."onchain_address" = $1
        ),
        (
            SELECT "id"
            FROM "divisions" d
            WHERE d."onchain_id" = $2
        ),
        $3,
        $4,
        $5
    ) ON CONFLICT (
        "onchain_officer_id",
        "division_id",
        "position_index"
    ) DO NOTHING;