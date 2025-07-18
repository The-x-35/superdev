#!/bin/bash

echo "Testing the API endpoints..."

echo -e "\n1. Testing signup endpoint:"
curl -X POST http://localhost:8080/signup \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser", "password": "password123", "role": "User"}'

echo -e "\n\n2. Testing signin endpoint:"
curl -X POST http://localhost:8080/signin \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser", "password": "password123", "role": "User"}'

echo -e "\n\n3. Testing create todo endpoint (with token):"
TOKEN=$(curl -s -X POST http://localhost:8080/signin \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser", "password": "password123", "role": "User"}' | jq -r '.token')

curl -X POST http://localhost:8080/todo \
  -H "Content-Type: application/json" \
  -H "Authorization: $TOKEN" \
  -d '{"text": "Test todo"}'

echo -e "\n\n4. Testing get todos endpoint:"
curl -X GET http://localhost:8080/todos \
  -H "Authorization: $TOKEN"

echo -e "\n\nDone!" 