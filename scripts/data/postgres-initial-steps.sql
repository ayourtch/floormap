DROP DATABASE {{pg_db}};
DROP SCHEMA {{pg_schema}};
DROP USER {{pg_user}};
CREATE SCHEMA IF NOT EXISTS {{pg_schema}};
CREATE USER {{pg_user}} PASSWORD '{{pg_pass}}';
ALTER USER {{pg_user}} CREATEDB;
GRANT ALL ON SCHEMA {{pg_schema}} TO {{pg_user}};
GRANT ALL ON ALL TABLES IN SCHEMA {{pg_schema}} TO {{pg_user}};
CREATE DATABASE {{pg_db}};
GRANT ALL ON DATABASE {{pg_db}} to {{pg_user}};
ALTER DATABASE {{pg_db}} OWNER TO {{pg_user}};


