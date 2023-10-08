INSERT INTO "onchain_documents" (
        "hash",
        "division_onchain_id",
        "submitter_address",
        "position_index",
        "signers_address"
    )
VALUES (
        $1,
        $2,
        $3,
        $4,
        $5
    );