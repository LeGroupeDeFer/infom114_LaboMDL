-- Your SQL goes here
CREATE TABLE watch_events(
    id          INT UNSIGNED        NOT NULL    AUTO_INCREMENT,
    post_id     INT UNSIGNED        NOT NULL,
    author_id   INT UNSIGNED        NOT NULL,

    event       TINYINT UNSIGNED    NOT NULL,
    time        DATETIME            NOT NULL    DEFAULT NOW(),
    comment     TEXT                NOT NULL,

    PRIMARY KEY (id),

    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (author_id) REFERENCES users(id),

    CONSTRAINT UNIQUE (post_id, event)
);