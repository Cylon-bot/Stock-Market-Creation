-- Data IO CAS init script

CREATE TABLE M1 (
    "candle_id" INTEGER PRIMARY KEY,
    "time" timestamptz NOT NULL,
    "open" float NOT NULL,
    "close" float NOT NULL,
    "high" float NOT NULL,
    "low" float NOT NULL
);

CREATE TABLE M5 (
    "candle_id" INTEGER PRIMARY KEY,
    "time" timestamptz NOT NULL,
    "open" float NOT NULL,
    "close" float NOT NULL,
    "high" float NOT NULL,
    "low" float NOT NULL
);

CREATE TABLE M15 (
    "candle_id" INTEGER PRIMARY KEY,*
    "time" timestamptz NOT NULL,
    "open" float NOT NULL,
    "close" float NOT NULL,
    "high" float NOT NULL,
    "low" float NOT NULL
);

CREATE TABLE H1 (
    "candle_id" INTEGER PRIMARY KEY,
    "time" timestamptz NOT NULL,
    "open" float NOT NULL,
    "close" float NOT NULL,
    "high" float NOT NULL,
    "low" float NOT NULL
);


CREATE TABLE H4 (
    "candle_id" INTEGER PRIMARY KEY,
    "time" timestamptz NOT NULL,
    "open" float NOT NULL,
    "close" float NOT NULL,
    "high" float NOT NULL,
    "low" float NOT NULL
);

CREATE TABLE D1 (
    "candle_id" INTEGER PRIMARY KEY,
    "time" timestamptz NOT NULL,
    "open" float NOT NULL,
    "close" float NOT NULL,
    "high" float NOT NULL,
    "low" float NOT NULL
);