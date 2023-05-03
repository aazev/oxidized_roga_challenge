#!/bin/bash
get_database_docker_id() {
    docker ps -qf "name=oxidized-roga-challenge-db"
}
# Load environment variables from .env file
pgid=$(get_database_docker_id)
export $(grep -v '^#' .env | xargs)
sleep 5

echo "Resetting database $DATABASE_NAME on $pgid using root:$MYSQL_ROOT_PASSWORD..."

# Check if the database exists
echo "Checking if the database exists..."
DBCHECK=$(docker exec "$pgid" mysql -h localhost -u root -p$MYSQL_ROOT_PASSWORD -e "SELECT SCHEMA_NAME FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = '$DATABASE_NAME'" | grep "$DATABASE_NAME")

if [ "$DBCHECK" != "$DATABASE_NAME" ]; then
  echo "Database does not exist, creating it..."
  docker exec "$pgid" mysql -h localhost -u root -p$MYSQL_ROOT_PASSWORD -e "CREATE DATABASE $DATABASE_NAME;"
else
  echo "Database exists, resetting it..."
  docker exec "$pgid" mysql -h localhost -u root -p$MYSQL_ROOT_PASSWORD -e "DROP DATABASE $DATABASE_NAME; CREATE DATABASE $DATABASE_NAME;"
fi

# Run migrations
echo "Running migrations..."
pnpm migration:run

echo "Done!"
