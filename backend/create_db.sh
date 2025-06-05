#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Database configuration
DB_USER="admin"
DB_PASSWORD="admin"
DB_NAME="db"
DB_HOST="localhost"
DB_PORT="5432"

echo -e "${GREEN}Starting PostgreSQL setup...${NC}"

# Check PostgreSQL installation
if ! command -v psql &> /dev/null; then
    echo -e "${RED}PostgreSQL is not installed. Installing...${NC}"
    sudo apt update
    sudo apt install -y postgresql postgresql-contrib
fi

# Ensure PostgreSQL is running
if ! pg_isready &> /dev/null; then
    echo "Starting PostgreSQL..."
    sudo systemctl start postgresql
fi

# Drop existing database and user for clean setup
echo "Cleaning up existing database and user..."
sudo -u postgres psql -c "DROP DATABASE IF EXISTS $DB_NAME;"
sudo -u postgres psql -c "DROP USER IF EXISTS $DB_USER;"

# Create user and database
echo "Creating database user..."
sudo -u postgres psql -c "CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD';"

echo "Creating database..."
sudo -u postgres psql -c "CREATE DATABASE $DB_NAME;"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"

# Apply schema
echo "Applying database schema..."
sudo -u postgres psql -d $DB_NAME -c "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";"

# Create tables
sudo -u postgres psql -d $DB_NAME << EOF
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS users_username_idx ON users(username);
CREATE INDEX IF NOT EXISTS users_email_idx ON users(email);
EOF

# Grant privileges
echo "Granting privileges..."
sudo -u postgres psql -d $DB_NAME -c "GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO $DB_USER;"
sudo -u postgres psql -d $DB_NAME -c "GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO $DB_USER;"
sudo -u postgres psql -d $DB_NAME -c "ALTER USER $DB_USER WITH SUPERUSER;"

# Update pg_hba.conf to allow password authentication
PG_HBA_FILE=$(sudo -u postgres psql -t -P format=unaligned -c 'SHOW hba_file;')
echo "Updating PostgreSQL authentication configuration..."
sudo sed -i '/^local.*all.*all.*peer/c\local   all             all                                     md5' $PG_HBA_FILE
sudo sed -i '/^host.*all.*all.*127.0.0.1.*ident/c\host    all             all             127.0.0.1/32            md5' $PG_HBA_FILE

# Restart PostgreSQL to apply authentication changes
echo "Restarting PostgreSQL..."
sudo systemctl restart postgresql

# Create or update .env file
echo "Updating environment configuration..."
echo "DATABASE_URL=postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME" > .env
echo "RUST_LOG=debug" >> .env

# Wait a moment for PostgreSQL to restart
sleep 2

echo -e "${GREEN}Database setup complete!${NC}"