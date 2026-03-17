CREATE TABLE IF NOT EXISTS zones (
    id                TEXT    PRIMARY KEY,
    name              TEXT    NOT NULL,
    start_pixel       INTEGER NOT NULL,
    end_pixel         INTEGER NOT NULL,
    transition_length INTEGER NOT NULL DEFAULT 8
);

CREATE TABLE IF NOT EXISTS effects (
    id     TEXT PRIMARY KEY,
    name   TEXT NOT NULL,
    script TEXT NOT NULL,
    params TEXT NOT NULL DEFAULT '[]'
);

CREATE TABLE IF NOT EXISTS scenes (
    id   TEXT PRIMARY KEY,
    name TEXT
);

INSERT OR IGNORE INTO scenes (id, name) VALUES ('__active__', NULL);

CREATE TABLE IF NOT EXISTS scene_layers (
    id         TEXT    PRIMARY KEY,
    scene_id   TEXT    NOT NULL REFERENCES scenes(id) ON DELETE CASCADE,
    effect_id  TEXT    NOT NULL,
    zone_id    TEXT    NOT NULL DEFAULT 'all',
    blend_mode TEXT    NOT NULL DEFAULT 'override',
    params     TEXT    NOT NULL DEFAULT '{}',
    enabled    INTEGER NOT NULL DEFAULT 1,
    position   INTEGER NOT NULL
);
