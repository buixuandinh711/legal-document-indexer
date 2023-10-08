INSERT INTO "onchain_positions" (
        "officer_address",
        "division_onchain_id",
        "position_index",
        "name",
        "role"
    )
VALUES ($1, $2, $3, $4, $5) ON CONFLICT (
        "officer_address",
        "division_onchain_id",
        "position_index"
    ) DO NOTHING;