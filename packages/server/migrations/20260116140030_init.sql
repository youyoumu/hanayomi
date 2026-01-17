CREATE TABLE dictionary (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    title TEXT NOT NULL,
    revision TEXT NOT NULL,
    author TEXT,
    description TEXT,
    attribution TEXT,
    url TEXT,

    source_language TEXT,
    target_language TEXT,
    frequency_mode TEXT,

    format INTEGER NOT NULL DEFAULT 3,
    sequenced BOOLEAN NOT NULL DEFAULT 0 CHECK (sequenced IN (0, 1)),
    minimum_yomitan_version TEXT,

    is_updatable BOOLEAN NOT NULL DEFAULT 0 CHECK (is_updatable IN (0, 1)),
    index_url TEXT,
    download_url TEXT,

    -- JSON Metadata (Stored as a String)
    tag_meta_json TEXT
);

CREATE TABLE dictionary_entry (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Foreign Key Constraint
    dictionary_id INTEGER NOT NULL,
    
    expression TEXT NOT NULL,
    reading TEXT NOT NULL,
    
    -- Vec<Definition> must be stored as a JSON string in SQLite
    definitions TEXT NOT NULL, 
    
    rules TEXT NOT NULL,
    score REAL NOT NULL DEFAULT 0.0,
    sequence INTEGER NOT NULL DEFAULT 0,
    definition_tags TEXT NOT NULL,
    expression_tags TEXT NOT NULL,

    FOREIGN KEY (dictionary_id) REFERENCES dictionary (id) ON DELETE CASCADE
);

CREATE TABLE definition_tag (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    dictionary_id INTEGER NOT NULL,

    name TEXT NOT NULL,
    category TEXT NOT NULL,
    "order" REAL NOT NULL DEFAULT 0, -- 'order' is a reserved keyword, use quotes
    notes TEXT NOT NULL,
    score REAL NOT NULL DEFAULT 0,

    FOREIGN KEY (dictionary_id) REFERENCES dictionary (id) ON DELETE CASCADE
);

--  ──────────────────────────── Speed Indices ────────────────────────────
CREATE INDEX idx_dictionary_entry__dictionary_id ON dictionary_entry(dictionary_id);
CREATE INDEX idx_dictionary_entry__expression ON dictionary_entry(expression);
CREATE INDEX idx_definition_tag__dictionary_id ON definition_tag(dictionary_id);

--  ──────────────────────── Automatic updated_at ─────────────────────
CREATE TRIGGER trig_dictionary__update_timestamp 
AFTER UPDATE ON dictionary 
BEGIN
    UPDATE dictionary SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE TRIGGER trig_dictionary_entry__update_timestamp 
AFTER UPDATE ON dictionary_entry 
BEGIN
    UPDATE dictionary_entry SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE TRIGGER trig_definition_tag__update_timestamp 
AFTER UPDATE ON definition_tag 
BEGIN
    UPDATE definition_tag SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

