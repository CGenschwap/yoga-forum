#!/bin/sh
# This script sets up a few basic entities
set -e

# First, create a user
curl -X 'POST' \
    -v \
    -H 'Content-Type: application/json' \
    'http://0.0.0.0:8080/v1/api/users/new' \
    -d \
'{
    "username": "TestUser",
    "password": "Testing"
}'

# Login as the user
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

# Post a story
curl -X 'POST' \
  'http://localhost:8080/v1/api/stories/new' \
  -v \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
  "title": "Some Title",
  "url": "http://example.com",
  "text": "Here is a body of response\n\nYou betcha!"
}'

# Post a comment on the story
curl -X 'POST' \
  'http://localhost:8080/v1/api/stories/1/comments/new' \
  -v \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
  "text": "string",
  "author_id": 1,
  "story_id": 1
}'

# Post a comment responding to the above comment
curl -X 'POST' \
  'http://localhost:8080/v1/api/stories/1/comments/new' \
  -v \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
  "text": "A child comment",
  "author_id": 1,
  "story_id": 1,
  "parent_id": 1
}'
