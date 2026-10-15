CREATE TABLE graduates (
    id VARCHAR NOT NULL PRIMARY KEY,
    full_name VARCHAR NOT NULL
);
CREATE TABLE positions (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    graduate_id VARCHAR NOT NULL,
    -- Jei nebus duodamas company_id, tai bus irasytas company_name, nes gali buti kompanija neuzregistruota ant LinkedIn
    company_id INTEGER,
    company_name VARCHAR,
    -- Role kompanijoje
    headline VARCHAR NOT NULL,

    start_month INTEGER NOT NULL,
    end_month INTEGER,

    from_course_id INTEGER,

    FOREIGN KEY (graduate_id) REFERENCES graduates(id)
        ON DELETE CASCADE,
    FOREIGN KEY (from_course_id) REFERENCES courses(id)
        ON UPDATE SET NULL
        ON DELETE SET NULL
);
CREATE TABLE graduate_courses (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    graduate_id VARCHAR NOT NULL,

    course_id INTEGER NOT NULL,

    FOREIGN KEY (graduate_id) REFERENCES graduates(id)
        ON DELETE CASCADE,
    FOREIGN KEY (course_id) REFERENCES courses(id)
        ON DELETE CASCADE
);
-- Mazdaug atskirti kursus, kad lengviau 
CREATE TABLE courses (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,

    name VARCHAR NOT NULL
);
-- Poziciju pavadinimai, kurie atitinka kursa
CREATE TABLE course_known_positions (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    course_id VARCHAR NOT NULL,

    name VARCHAR NOT NULL,

    FOREIGN KEY (course_id) REFERENCES courses(id)
        ON DELETE CASCADE
);
-- Skirtinga lentele, kad nereiketu filtruoti ir tureti kelius NULL reiksmes
CREATE TABLE imported_graduates (
    id VARCHAR NOT NULL PRIMARY KEY
);