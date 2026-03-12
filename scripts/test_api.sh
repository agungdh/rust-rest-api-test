#!/bin/bash

BASE_URL="http://localhost:3000/api"

echo "=========================================="
echo "Employee API Test Script"
echo "=========================================="

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Helper function to extract UUID from JSON
extract_uuid() {
    echo "$1" | grep -o '"uuid":"[^"]*"' | head -1 | cut -d'"' -f4
}

# Test Department CRUD
echo -e "\n${GREEN}[DEPARTMENT CRUD]${NC}"

# Create Department
echo -e "\n--- Create Department ---"
DEPT_RESPONSE=$(curl -s -X POST "$BASE_URL/departments" \
  -H "Content-Type: application/json" \
  -d '{"name": "Engineering"}')
echo "$DEPT_RESPONSE"
DEPT_UUID=$(extract_uuid "$DEPT_RESPONSE")
echo "Created Department UUID: $DEPT_UUID"

# Get All Departments
echo -e "\n--- Get All Departments ---"
curl -s "$BASE_URL/departments"
echo

# Get Department by UUID
echo -e "\n--- Get Department by UUID ---"
curl -s "$BASE_URL/departments/$DEPT_UUID"
echo

# Update Department
echo -e "\n--- Update Department ---"
curl -s -X PUT "$BASE_URL/departments/$DEPT_UUID" \
  -H "Content-Type: application/json" \
  -d '{"name": "Engineering Updated"}'
echo

# Test Employee CRUD
echo -e "\n${GREEN}[EMPLOYEE CRUD]${NC}"

# Create Employee
echo -e "\n--- Create Employee ---"
EMP_RESPONSE=$(curl -s -X POST "$BASE_URL/employees" \
  -H "Content-Type: application/json" \
  -d "{\"name\": \"John Doe\", \"email\": \"john@test.com\", \"position\": \"Software Engineer\", \"salary\": 5000000, \"department_uuid\": \"$DEPT_UUID\"}")
echo "$EMP_RESPONSE"
EMP_UUID=$(extract_uuid "$EMP_RESPONSE")
echo "Created Employee UUID: $EMP_UUID"

# Get All Employees
echo -e "\n--- Get All Employees ---"
curl -s "$BASE_URL/employees"
echo

# Get Employees by Department
echo -e "\n--- Get Employees by Department ---"
curl -s "$BASE_URL/employees?department_uuid=$DEPT_UUID"
echo

# Get Employee by UUID
echo -e "\n--- Get Employee by UUID ---"
curl -s "$BASE_URL/employees/$EMP_UUID"
echo

# Update Employee
echo -e "\n--- Update Employee ---"
curl -s -X PUT "$BASE_URL/employees/$EMP_UUID" \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe Updated", "salary": 6000000}'
echo

# Delete Employee
echo -e "\n--- Delete Employee ---"
curl -s -X DELETE "$BASE_URL/employees/$EMP_UUID"
echo

# Verify Employee Deleted
echo -e "\n--- Verify Employee Deleted ---"
curl -s "$BASE_URL/employees/$EMP_UUID"
echo

# Delete Department
echo -e "\n--- Delete Department ---"
curl -s -X DELETE "$BASE_URL/departments/$DEPT_UUID"
echo

# Verify Department Deleted
echo -e "\n--- Verify Department Deleted ---"
curl -s "$BASE_URL/departments/$DEPT_UUID"
echo

echo -e "\n${GREEN}=========================================="
echo "All tests completed!"
echo "==========================================${NC}"
