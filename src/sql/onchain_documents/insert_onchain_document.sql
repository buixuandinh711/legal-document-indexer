INSERT INTO "onchain_documents" (
        "document_content_hash",
        "number",
        "name",
        "doc_type",
        "division_id",
        "published_timestamp",
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
        $8,
        $9
    ) ON CONFLICT ("document_content_hash") DO NOTHING;