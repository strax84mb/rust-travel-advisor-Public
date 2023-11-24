CREATE TABLE routes (
    id     BIGINT NOT NULL AUTO_INCREMENT,
    start  BIGINT NOT NULL,
    finish BIGINT NOT NULL,
    price  BIGINT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_start_city  FOREIGN KEY (start)  REFERENCES cities(id),
    CONSTRAINT fk_finish_city FOREIGN KEY (finish) REFERENCES cities(id)
);