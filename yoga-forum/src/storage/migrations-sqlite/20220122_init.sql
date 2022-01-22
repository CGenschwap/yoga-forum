CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL
    , username TEXT NOT NULL UNIQUE
    , normalized_username TEXT NOT NULL UNIQUE
    , created_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL
    , updated_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL
    CHECK(
        length("username") <= 80
    )
);

CREATE TRIGGER update_updated_at_users
AFTER UPDATE ON users
BEGIN
    UPDATE users SET updated_at = (strftime('%s','now')) WHERE id = NEW.id;
END;

-- Separate from the users table to limit blast radius
-- of a botched users table query
CREATE TABLE passwords (
    id INTEGER PRIMARY KEY NOT NULL
    , user_id INTEGER NOT NULL
    , hash TEXT NOT NULL
    , created_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL
    , updated_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL

    , FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TRIGGER update_updated_at_passwords
AFTER UPDATE ON passwords
BEGIN
    UPDATE passwords SET updated_at = (strftime('%s','now')) WHERE id = NEW.id;
END;

CREATE TABLE stories (
    id INTEGER PRIMARY KEY NOT NULL
    , title TEXT NOT NULL
    , url TEXT
    , text TEXT
    , author_id INTEGER NOT NULL
    , created_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL
    , updated_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL

    , FOREIGN KEY(author_id) REFERENCES users(id)
    CHECK(
        length("text") <= 1024 AND
        length("url") <= 80 AND
        length("title") <= 80
    )
);

CREATE TRIGGER update_updated_at_stories
AFTER UPDATE ON stories
BEGIN
    UPDATE stories SET updated_at = (strftime('%s','now')) WHERE id = NEW.id;
END;

CREATE TABLE comments (
    id INTEGER PRIMARY KEY NOT NULL
    , text TEXT NOT NULL
    , author_id INTEGER NOT NULL
    , story_id INTEGER NOT NULL
    , parent_id INTEGER
    , created_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL
    , updated_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL

    , FOREIGN KEY(author_id) REFERENCES users(id)
    , FOREIGN KEY(story_id) REFERENCES stories(id)
    , FOREIGN KEY(parent_id) REFERENCES comments(id)
    CHECK(
        length("text") <= 1024
    )
);

CREATE TRIGGER update_updated_at_comments
AFTER UPDATE ON comments
BEGIN
    UPDATE comments SET updated_at = (strftime('%s','now')) WHERE id = NEW.id;
END;
