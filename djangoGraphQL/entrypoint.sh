#!/bin/sh
set -e

# Default values (adjust as needed)
DB_HOST=${DB_HOST:-postgres}        # Use the service name 'db' by default
DB_PORT=${DB_PORT:-5432}      # Default port for PostgreSQL

echo "Waiting for PostgreSQL at $DB_HOST:$DB_PORT..."
# Loop until we can connect to the DB host/port using netcat
while ! nc -z "$DB_HOST" "$DB_PORT"; do
  echo "PostgreSQL is not available yet - sleeping"
  sleep 1
done

echo "PostgreSQL is up - running migrations..."
python manage.py migrate --noinput

echo "Starting the Django application..."
exec "$@"