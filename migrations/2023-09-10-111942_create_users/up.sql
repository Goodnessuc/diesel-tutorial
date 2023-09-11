-- Your SQL goes here

-- Your SQL goes here

-- CREATE TABLE "human"
-- (
--     "id"         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
--     "first_name" TEXT    NOT NULL,
--     "last_name"  TEXT    NOT NULL,
--     "age"        INTEGER NOT NULL,
--     "username"   TEXT,
--     "email"      TEXT,
--     "location"   TEXT
-- );





-- Create the item_type table
CREATE TABLE IF NOT EXISTS item_type
(
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

-- Create the item table with foreign key reference to item_type
CREATE TABLE IF NOT EXISTS item
(
    id            INTEGER PRIMARY KEY,
    name          TEXT NOT NULL,
    item_type_id  INTEGER NOT NULL,
    acquired_time TIMESTAMP NOT NULL,
    FOREIGN KEY (item_type_id) REFERENCES item_type(id)
);
