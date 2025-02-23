#!/bin/bash

set -e  # Exit on error

# Load environment variables from .env file
if [ -f .env ]; then
    set -a  # Export all variables
    source .env
    set +a
else
    echo ".env file not found!"
    exit 1
fi

# Ensure DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "DATABASE_URL is not set!"
    exit 1
fi

# Correctly extract PostgreSQL connection details using regex
DB_USER=$(echo $DATABASE_URL | sed -E 's|.*://([^:]+):.*|\1|')
DB_PASS=$(echo $DATABASE_URL | sed -E 's|.*://[^:]+:([^@]+)@.*|\1|')
DB_HOST=$(echo $DATABASE_URL | sed -E 's|.*@([^:/]+).*|\1|')
DB_NAME=$(echo $DATABASE_URL | sed -E 's|.*/([^/?]+).*|\1|')

export PGPASSWORD=$DB_PASS

echo "Parsed Database Credentials:"
echo "  User: $DB_USER"
echo "  Host: $DB_HOST"
echo "  Database: $DB_NAME"

# Ensure DB_USER is correctly extracted
if [[ -z "$DB_USER" || -z "$DB_HOST" || -z "$DB_NAME" ]]; then
    echo "Error: Failed to parse database credentials."
    exit 1
fi

echo "Waiting for PostgreSQL to be ready..."
until psql -h $DB_HOST -U $DB_USER -d postgres -c '\q' 2>/dev/null; do
    sleep 1
done

# Check if the database exists
DB_EXISTS=$(psql -h $DB_HOST -U $DB_USER -d postgres -tAc "SELECT 1 FROM pg_database WHERE datname='$DB_NAME'")

if [ "$DB_EXISTS" != "1" ]; then
    echo "Database '$DB_NAME' does not exist. Creating it now..."
    psql -h $DB_HOST -U $DB_USER -d postgres -c "CREATE DATABASE $DB_NAME;"
else
    echo "Database '$DB_NAME' already exists."
fi

echo "Running migrations now..."
sqlx migrate run

unset PGPASSWORD  # Remove password from environment for security

echo "Setup done!"
