TOKEN=`curl -X 'POST' \
    -v \
    -H 'Content-Type: application/json' \
    'http://0.0.0.0:8080/v1/api/users/login' \
    -d \
'{
    "user_id": 1,
    "password": "Testing"
}'`

TOKEN=`echo $TOKEN | jq -r .token`

echo $TOKEN

curl -X 'POST' \
  'http://localhost:8080/v1/api/stories/new' \
  -v \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
  "title": "string",
  "url": "http://test.com",
  "description": "string"
}'

