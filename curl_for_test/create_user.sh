#!/bin/bash

API_URL="http://localhost:8000/api/users"

# Créer un utilisateur
create_user() {
  echo "Creating user..."
  response=$(curl -s -X POST $API_URL \
    -H "Content-Type: application/json" \
    -d '{
      "first_name": "John",
      "last_name": "Doe",
      "password_hash": "hashed_password_example",
      "address": "123 Main St, Springfield",
      "phone_number": "+123456789",
      "ssn": "123-45-6789"
    }')

  echo "Response: $response"
  user_id=$(echo $response | jq -r '.id')
  echo "User created with ID: $user_id"
}

# Modifier un utilisateur
update_user() {
  local user_id=$1

  echo "Updating user with ID $user_id..."
  response=$(curl -s -X PUT "$API_URL/$user_id" \
    -H "Content-Type: application/json" \
    -d '{
      "first_name": "Jane",
      "last_name": "Doe",
      "address": "456 Elm St, Springfield",
      "phone_number": "+987654321"
    }')

  echo "Response: $response"
}

# Supprimer un utilisateur
delete_user() {
  local user_id=$1

  echo "Deleting user with ID $user_id..."
  response=$(curl -s -X DELETE "$API_URL/$user_id")

  echo "Response: $response"
}

# Exécuter les fonctions
create_user
# Wait for 2 seconds to ensure user creation
sleep 2
user_id=$(echo $response | jq -r '.id')

if [ "$user_id" != "null" ]; then
  update_user $user_id
  sleep 2
  delete_user $user_id
else
  echo "Failed to create user."
fi
