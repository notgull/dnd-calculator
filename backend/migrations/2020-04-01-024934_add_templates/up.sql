CREATE TABLE Moves (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    hit_type INTEGER NOT NULL,
    hit_radius INTEGER,
    dice_count INTEGER NOT NULL,
    dice_type INTEGER NOT NULL,
    dice_modifier INTEGER NOT NULL,
    stat_boost INTEGER NOT NULL,
    saving_throw INTEGER,
    effect INTEGER,
    effect_severity INTEGER
);

CREATE TABLE Items (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE Templates (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    health INTEGER NOT NULL,
    armor_class INTEGER NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE MoveUsage (
    id SERIAL PRIMARY KEY,
    move_id INTEGER REFERENCES Moves(id),
    template_id INTEGER REFERENCES Templates(id)
);

CREATE TABLE ItemsCarried (
    id SERIAL PRIMARY KEY,
    item_id INTEGER REFERENCES Items(id),
    template_id INTEGER REFERENCES Templates(id)
);
