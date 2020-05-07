-- Your SQL goes here

CREATE TABLE poll_answers (
    id INT UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    post_id INT UNSIGNED NOT NULL,
    answer NVARCHAR(250) NOT NULL,
    CONSTRAINT FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT unique_polls_post_answer UNIQUE (post_id, answer)
);

CREATE TABLE users_poll_answers (
    answer_id INT UNSIGNED NOT NULL,
    user_id INT UNSIGNED NOT NULL,
    PRIMARY KEY (answer_id,user_id),
    CONSTRAINT FOREIGN KEY (answer_id) REFERENCES poll_answers(id),
    CONSTRAINT FOREIGN KEY (user_id) REFERENCES users(id)
);