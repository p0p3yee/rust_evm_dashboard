-- Your SQL goes here

CREATE TABLE settings (
    `selected_endpoint_id` integer NOT NULL PRIMARY KEY,
    FOREIGN KEY (`selected_endpoint_id`) REFERENCES endpoints (`id`)
);