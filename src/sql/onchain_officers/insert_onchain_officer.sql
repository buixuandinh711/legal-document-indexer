INSERT INTO "onchain_officers" (
        "address",
        "name",
        "date_of_birth",
        "sex",
        "status"
    )
VALUES ($1, $2, $3, $4, $5) ON CONFLICT ("address") DO NOTHING;