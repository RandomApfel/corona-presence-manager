CREATE TABLE users (
    token TEXT PRIMARY KEY,
    full_name TEXT,
    admin_view BOOL
);

CREATE TABLE presences (
    quarter INT,
    token TEXT,
    comment TEXT
);
