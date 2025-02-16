CREATE TABLE IF NOT EXISTS item
(
    item_id     INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL,
    description TEXT                NOT NULL,
    damages     TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS tag
(
    tag_id      INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL,
    generic     BOOLEAN             NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS item_tag
(
    tag_id      INTEGER REFERENCES tag(tag_id) ON DELETE CASCADE NOT NULL,
  	item_id		INTEGER REFERENCES item(item_id) on DELETE CASCADE NOT NULL,
  	PRIMARY KEY (tag_id, item_id)
);

CREATE TABLE IF NOT EXISTS image
(
    image_id    INTEGER PRIMARY KEY NOT NULL,
    item_id     INTEGER REFERENCES item(item_id) on DELETE CASCADE NOT NULL,
    description TEXT                NOT NULL,
    image_name  TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS user(
    user_id     INTEGER PRIMARY KEY NOT NULL,
    username    TEXT                NOT NULL,
    password    TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS borrow(
    borrow_id    INTEGER PRIMARY KEY NOT NULL,
    item_id      INTEGER REFERENCES item(item_id) on DELETE CASCADE NOT NULL,
    user_id      INTEGER REFERENCES user(user_id) on DELETE CASCADE NOT NULL   
    damage       BOOLEAN                NOT NULL DEFAULT 0,
    borrrow_date DATETIME               NOT NULL DEFAULT CURRENT_TIMESTAMP,
    due_date     DATETIME               NOT NULL DEFAULT CURRENT_TIMESTAMP,
    returned     BOOLEAN                NOT NULL DEFAULT 0,
    reason       INTEGER                NOT NULL DEFAULT 0,
    returned_date DATETIME
);