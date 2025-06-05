#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Database configuration
DB_USER="admin"
DB_PASSWORD="admin"
DB_NAME="db"

# API URL
API_URL="http://localhost:8080"

# Disable psql pager
export PAGER=

# Function to check database
check_database() {
    local query=$1
    local expected_count=$2
    local description=$3

    echo -e "\n${GREEN}Checking database: $description${NC}"

    result=$(PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -d $DB_NAME -t -A -c "$query" | tr -d '[:space:]')

    if [ "$result" -eq "$expected_count" ]; then
        echo -e "${GREEN}✓ Database check passed: Found $result records${NC}"
        return 0
    else
        echo -e "${RED}✗ Database check failed: Expected $expected_count, found $result${NC}"
        return 1
    fi
}

# Function to show user details
show_user_details() {
    local username=$1
    echo -e "\n${GREEN}Fetching details for user: $username${NC}"

    PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -d $DB_NAME -X -A -F $'\t' -c "
        SELECT
            id,
            username,
            email,
            created_at
        FROM users
        WHERE username = '$username';"
}

# Function to test endpoint
test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local expected_status=$4
    local description=$5

    echo -e "\n${GREEN}Testing: $description${NC}"

    if [ -n "$data" ]; then
        response=$(curl -s -w "\n%{http_code}" -X $method "$API_URL$endpoint" \
            -H "Content-Type: application/json" \
            -d "$data")
    else
        response=$(curl -s -w "\n%{http_code}" -X $method "$API_URL$endpoint")
    fi

    status_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed \$d)

    if [ "$status_code" -eq "$expected_status" ]; then
        echo -e "${GREEN}✓ Status code $status_code matches expected $expected_status${NC}"
        echo "Response body: $body"
    else
        echo -e "${RED}✗ Status code $status_code does not match expected $expected_status${NC}"
        echo "Response body: $body"
        return 1
    fi
}

# Clean database before testing
echo -e "\n${GREEN}Cleaning database before testing...${NC}"
PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -d $DB_NAME -X -c "TRUNCATE users CASCADE;"

# Initial database check
echo -e "\n${GREEN}Initial database state:${NC}"
check_database "SELECT COUNT(*) FROM users" 0 "Database should be empty initially"

# Wait for the server to start
echo "Waiting for server to start..."
sleep 2

# Test invalid endpoint
test_endpoint "GET" "/" "" 404 "Invalid endpoint (should return 404)"

# Test profile endpoint (unauthorized)
test_endpoint "GET" "/profile" "" 200 "Profile endpoint"

# Test login with invalid credentials
test_endpoint "POST" "/login" '{"username":"invalid","password":"invalid"}' 401 "Login with invalid credentials"

# Create test user
test_endpoint "POST" "/register" '{"username":"test","email":"test@example.com","password":"password123"}' 200 "Register new user"

# Verify user was created in database
check_database "SELECT COUNT(*) FROM users WHERE username = 'test'" 1 "User 'test' should exist in database"

# Show user details in a clean format
echo -e "\n${GREEN}User details:${NC}"
PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -d $DB_NAME -X -A -F $'\t' -c "
    SELECT id, username, email, created_at
    FROM users
    WHERE username = 'test';" | column -t -s $'\t'

# Test login with valid credentials
test_endpoint "POST" "/login" '{"username":"test","password":"password123"}' 200 "Login with valid credentials"

# Try to create duplicate user
test_endpoint "POST" "/register" '{"username":"test","email":"test@example.com","password":"password123"}' 500 "Attempt to create duplicate user (should fail)"

# Verify still only one user exists
check_database "SELECT COUNT(*) FROM users" 1 "Database should still have only one user"

# Show final database state in a clean format
echo -e "\n${GREEN}Final database state:${NC}"
PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -d $DB_NAME -X -A -F $'\t' -c "
    SELECT id, username, email, created_at
    FROM users;" | column -t -s $'\t'

echo -e "\n${GREEN}All tests completed!${NC}"