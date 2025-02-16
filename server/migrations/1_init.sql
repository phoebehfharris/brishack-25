CREATE TABLE IF NOT EXISTS items
(
    id     INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL,
    description TEXT                NOT NULL,
    damages     TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS tags
(
    id      INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL,
    generic     BOOLEAN             NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS item_tags
(
    tag_id      INTEGER REFERENCES tag(id) ON DELETE CASCADE NOT NULL,
  	item_id		INTEGER REFERENCES item(id) on DELETE CASCADE NOT NULL,
  	PRIMARY KEY (tag_id, item_id)
);

CREATE TABLE IF NOT EXISTS images
(
    id    INTEGER PRIMARY KEY NOT NULL,
    item_id     INTEGER REFERENCES item(id) on DELETE CASCADE NOT NULL,
    description TEXT                NOT NULL,
    image_name  TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS users(
    id     INTEGER PRIMARY KEY NOT NULL,
    username    TEXT                NOT NULL,
    password    TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS borrows(
    id    INTEGER PRIMARY KEY NOT NULL,
    item_id      INTEGER REFERENCES item(id) on DELETE CASCADE NOT NULL,
    user_id      INTEGER REFERENCES user(id) on DELETE CASCADE NOT NULL,   
    damage       BOOLEAN                NOT NULL DEFAULT 0,
    borrrow_date DATETIME               NOT NULL DEFAULT CURRENT_TIMESTAMP,
    due_date     DATETIME               NOT NULL DEFAULT CURRENT_TIMESTAMP,
    returned     BOOLEAN                NOT NULL DEFAULT 0,
    reason       INTEGER                NOT NULL DEFAULT 0,
    returned_date DATETIME
);