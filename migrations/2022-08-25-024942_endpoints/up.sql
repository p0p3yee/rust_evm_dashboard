CREATE TABLE endpoints (
    `id` integer NOT NULL PRIMARY KEY,
    `name` text NOT NULL,
    `url` text NOT NULL,
    `symbol` text NOT NULL,
    `chain_id` text NOT NULL,
    `explorer_url` text
)