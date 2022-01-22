curl -X 'POST' \
    -v \
    -H 'Content-Type: application/json' \
    'http://0.0.0.0:8080/api/v1/users/new' \
    -d \
'{
    "username": "TestUser",
    "password": "Testing"
}'

