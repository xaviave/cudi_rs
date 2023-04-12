# PostgreSQL in CUDI

PostgreSQL is the database use in CUDI to save file url (local or online).
Tags and file formats are saved in their tables allow advanced filtering for a better CUDI display.
[Diesel](https://diesel.rs/guides/relations.html) is used to managed the SQL query.
`populate_db.py` will be the full db manager, to auto tag and gather data from local data.
For API data, medias will not be tagged immediately, a process will be created on the fly.

## Create user and DB

    	CREATE ROLE gmx;
    	CREATE DATABASE cudi_db;
    	CREATE USER gmx WITH PASSWORD '1234';
    	ALTER ROLE gmx SET default_transaction_isolation TO 'read committed';
    	GRANT ALL PRIVILEGES ON DATABASE cudi_db TO gmx;
    	\q

## Create tables

refer to migrations `2023-04-06-135720_create_db/up.rs`:

    	CREATE TABLE format (
    		id SERIAL PRIMARY KEY,
    		name VARCHAR NOT NULL UNIQUE
    	);

    	CREATE TABLE tag (
    		id SERIAL PRIMARY KEY,
    		name VARCHAR NOT NULL UNIQUE
    	);

    	CREATE TABLE media (
    		url VARCHAR NOT NULL UNIQUE,
    		format_id INTEGER REFERENCES format(id),
    		tag_id INTEGER REFERENCES tag(id),
    		PRIMARY KEY(format_id, tag_id)
    	);
