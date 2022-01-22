TOKEN=""

hey -m POST -H 'Content-Type: application/json' -H "Authorization: Bearer ${TOKEN}" -d '{"text": "test", "parent_id": 1}' http://0.0.0.0:8080/v1/api/stories/1/comments/new
