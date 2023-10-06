SELECT "id",
    "password",
    "onchain_address"
FROM "officers"
WHERE "username" = $1;