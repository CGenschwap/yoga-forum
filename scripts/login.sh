curl -X 'POST' \
    -v \
    -H 'Content-Type: application/json' \
    'http://0.0.0.0:8080/v1/api/users/login' \
    -d \
'{
    "user_id": 1,
    "password": "Testing"
}'

