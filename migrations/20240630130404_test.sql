-- Data IO CAS init script

CREATE TABLE M1 (
    "candle_id" INTEGER PRIMARY KEY,
    "open" float,
    "close" float,
    "high" float,
    "low" float
);

CREATE TABLE M5 (
    "candle_id" INTEGER PRIMARY KEY,
    "open" float,
    "close" float,
    "high" float,
    "low" float
);

CREATE TABLE M15 (
    "candle_id" INTEGER PRIMARY KEY,
    "open" float,
    "close" float,
    "high" float,
    "low" float
);

CREATE TABLE H1 (
    "candle_id" INTEGER PRIMARY KEY,
    "open" float,
    "close" float,
    "high" float,
    "low" float
);


CREATE TABLE H4 (
    "candle_id" INTEGER PRIMARY KEY,
    "open" float,
    "close" float,
    "high" float,
    "low" float
);

CREATE TABLE D1 (
    "candle_id" INTEGER PRIMARY KEY,
    "open" float,
    "close" float,
    "high" float,
    "low" float
);