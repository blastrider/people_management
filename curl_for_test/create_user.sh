#!/bin/bash

curl -X POST http://localhost:8000/api/users \
-H "Content-Type: application/json" \
-d '{
    "first_name": "John",
    "last_name": "Doe",
    "password_hash": "hashed_password_example",
    "address": "123 Main St, Springfield",
    "phone_number": "+123456789",
    "ssn": "123-45-6789"
}'