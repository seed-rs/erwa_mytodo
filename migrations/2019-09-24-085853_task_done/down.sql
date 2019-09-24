PRAGMA foreign_keys=off;

ALTER TABLE task RENAME TO _task_old;

CREATE TABLE task (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO task (id, title)
      SELECT id, title
      FROM _task_old;

DROP TABLE _task_old;

PRAGMA foreign_keys=on;