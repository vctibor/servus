#!/bin/sh

echo "ADD USER"

curl -vi -d '{"name":"a", "email":"a@a.a"}' -H "Content-Type: application/json" -X POST http://localhost:8080/api/user/create
curl -vi -d '{"name":"a"}' -H "Content-Type: application/json" -X POST http://localhost:8080/api/user/create



printf "\n\nGET USERS\n\n"

curl -X GET http://localhost:8080/api/user/list | python -m json.tool