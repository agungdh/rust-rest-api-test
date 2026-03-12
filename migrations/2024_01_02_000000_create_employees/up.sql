CREATE TABLE employees (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    position TEXT NOT NULL,
    salary INTEGER NOT NULL,
    department_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (department_id) REFERENCES departments(id)
);
