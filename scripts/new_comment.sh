TOKEN=`curl -X 'POST' \
    -v \
    -H 'Content-Type: application/json' \
    'http://0.0.0.0:8080/api/v1/users/login' \
    -d \
'{
    "user_id": 1,
    "password": "Testing"
}'`

TOKEN=`echo $TOKEN | jq -r .token`

echo $TOKEN

curl -X 'POST' \
  'http://localhost:8080/api/v1/comments/new' \
  -v \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
  "text": "string",
  "author_id": 1,
  "story_id": 1
}'

