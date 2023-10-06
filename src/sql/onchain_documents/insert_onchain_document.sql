INSERT INTO "submitted_documents" (
        "hash",
        "division_id",
        "onchain_officer_id",
        "position_index",
        "signer_onchain_id"
    )
VALUES (
        $1,
        (
            SELECT "id"
            FROM "divisions"
            WHERE "divisions"."onchain_id" = $2
        ),
        $3,
        $4,
        $5
    );