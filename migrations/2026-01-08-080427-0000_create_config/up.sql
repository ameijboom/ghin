CREATE TABLE config(
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    installation_directory VARCHAR NOT NULL DEFAULT "/usr/local/bin",
    temp_directory VARCHAR NOT NULL DEFAULT "/tmp"
)
