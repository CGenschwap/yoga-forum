
TOKEN=`curl -X 'POST' \
  'http://localhost:8080/api/v1/users/login' \
  -v
`

curl -X 'POST' \
  'http://localhost:8080/api/v1/stories/new' \
  -v \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
  "title": "string",
  "url": "string",
  "description": "string"
}'

