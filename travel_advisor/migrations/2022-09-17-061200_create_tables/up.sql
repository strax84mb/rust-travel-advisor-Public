CREATE TABLE users (
    id    BIGINT       NOT NULL AUTO_INCREMENT,
    email VARCHAR(50)  NOT NULL,
    pass  VARCHAR(50)  NOT NULL,
    roles VARCHAR(100) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE cities (
    id     BIGINT      NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(30) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE comments (
    id      BIGINT       NOT NULL AUTO_INCREMENT,
    user_id BIGINT       NOT NULL,
    city_id BIGINT       NOT NULL,
    `text`  VARCHAR(250) NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_comment_user FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT fk_comment_city FOREIGN KEY (city_id) REFERENCES cities(id)
);

CREATE TABLE airports (
    id      BIGINT      NOT NULL AUTO_INCREMENT,
    city_id BIGINT      NOT NULL,
    `name`  VARCHAR(50) NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_airport_city FOREIGN KEY (city_id) REFERENCES cities(id)
);
