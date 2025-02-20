CREATE TABLE groups (
    id text not null,
    name text not null,
    created_at timestamptz not null
);

INSERT INTO groups (id, name, created_at) VALUES ('abc123', 'Test Group', CURRENT_TIMESTAMP);
