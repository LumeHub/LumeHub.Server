CREATE TABLE IF NOT EXISTS state_stack (
    position    INTEGER PRIMARY KEY AUTOINCREMENT,
    layers_json TEXT    NOT NULL,
    device_json TEXT    NOT NULL,
    pushed_at   INTEGER NOT NULL
);
