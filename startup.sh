#!/bin/bash
set -e

echo "Waiting for database to be ready..."
until pg_isready -U anjal -h db; do
    sleep 2
done

echo "Running migrations using sqlx..."
sqlx migrate run

echo "Database setup complete."
