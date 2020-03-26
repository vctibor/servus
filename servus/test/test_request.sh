#!/bin/sh

printf "\nADD USER\n"

USER_ID=$(curl -s -d '{"name":"a", "email":"a@a.a"}' -H "Content-Type: application/json" -X POST http://localhost:8080/api/user/create | jq .id | tr -d \")

echo $USER_ID

# curl -vi -d '{"name":"a"}' -H "Content-Type: application/json" -X POST http://localhost:8080/api/user/create


printf "\nLIST USERS\n"

curl -s -X GET http://localhost:8080/api/user/list | jq .


printf "\nGET USER\n"

curl -s -X GET http://localhost:8080/api/user/get/$USER_ID | jq .


printf "\nUPDATE USER\n"

curl -s -d '{"name":"b", "email":"b@a.a"}' -H "Content-Type: application/json" -X POST http://localhost:8080/api/user/update/$USER_ID

curl -s -X GET http://localhost:8080/api/user/list | jq .


printf "\nDELETE USER\n"

curl -s -X POST http://localhost:8080/api/user/delete/$USER_ID
