FROM postgres:16

# Install dependencies
RUN apt-get update && apt-get install -y curl libpq-dev pkg-config openssl

WORKDIR /app

# Copy migration files (optional, just in case)
COPY migrations /app/migrations

# Set the default command to run PostgreSQL
CMD ["postgres"]
