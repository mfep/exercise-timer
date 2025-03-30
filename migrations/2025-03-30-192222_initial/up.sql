-- Your SQL goes here
CREATE TABLE training(
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    hidden INTEGER NOT NULL DEFAULT 0,
    sets INTEGER NOT NULL,
    warmup_s INTEGER NOT NULL,
    exercise_s INTEGER NOT NULL,
    prepare_s INTEGER NOT NULL
)
