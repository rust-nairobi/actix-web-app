#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
	CREATE USER dbuser;
	CREATE DATABASE cloudserver;
	GRANT ALL PRIVILEGES ON DATABASE cloudserver TO docker;
EOSQL