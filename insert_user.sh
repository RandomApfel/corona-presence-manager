#!/bin/bash

echo Enter a name
read -r NAME

TOKEN=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 8 | head -n 1)
echo Enter a token \($TOKEN\)
read -r NEW_TOKEN

if [[ "$NEW_TOKEN" != "" ]]
then
    TOKEN="$NEW_TOKEN"
fi

ADMIN=0
echo Admin? \(No\)
read -a NEW_ADMIN

if [[ "$NEW_ADMIN" == "y" ]] || [[ "$NEW_ADMIN" == "t" ]]
then
    ADMIN=1
fi

echo Enter user "$NAME" with token "$TOKEN" as admin: $ADMIN? \(Enter or Ctrl+C\)
read

echo 'INSERT INTO users (token, full_name, admin_view) VALUES ("'$TOKEN'", "'$NAME'", '$ADMIN')' | sqlite3 db.sqlite3
