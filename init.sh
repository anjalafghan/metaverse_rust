#!/bin/bash
set -e

# Load environment variables
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo ".env file not found!"
    exit 1
fi

echo "Waiting for database to be ready..."
until PGPASSWORD="$PGPASSWORD" psql -h "$PGHOST" -U "$PGUSER" -d postgres -c '\q' 2>/dev/null; do
    sleep 2
done

echo "Running migrations using sqlx..."
sqlx migrate run

echo "Database setup complete."
