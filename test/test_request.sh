#!/bin/sh

echo "ADD USER"

curl -vi -d '{"name":"a", "email":"a@a.a"}' -H "Content-Type: application/json" -X POST http://localhost:8080/extractor
curl -vi -d '{"name":"a"}' -H "Content-Type: application/json" -X POST http://localhost:8080/extractor



printf "\n\nGET USERS\n\n"

curl -X GET http://localhost:8080/get_users | python -m json.tool