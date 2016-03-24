DROP TABLE IF EXISTS posts;

CREATE TABLE posts (
    id      INT PRIMARY KEY,
    title   VARCHAR NOT NULL,
    date    DATETIME NOT NULL,
    html    VARCHAR NOT NULL,
    summary VARCHAR(250) NOT NULL,
    url     VARCHAR(100) NOT NULL
);
