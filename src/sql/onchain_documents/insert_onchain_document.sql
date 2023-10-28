INSERT INTO "onchain_documents" (
        "document_content_hash",
        "number",
        "name",
        "division_id",
        "publishedTimestamp",
        "publisher_address",
        "publisher_division_id",
        "publisher_position_index"
    )
VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8
    ) ON CONFLICT ("document_content_hash") DO NOTHING;