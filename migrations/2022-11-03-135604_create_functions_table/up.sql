create table functions (
       id serial PRIMARY KEY,
       ref_name VARCHAR UNIQUE NOT NULL,
       route VARCHAR NOT NULL,
       project_id INT NOT NULL,
       CONSTRAINT fk_project FOREIGN KEY(project_id) REFERENCES projects(id)
);
