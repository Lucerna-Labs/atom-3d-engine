#!/bin/bash
cd /opt/app
docker exec app-db-1 mariadb -upixelfed -p'eXseRRmQGN-FagQfJSG1z' pixelfed_prod -e "SELECT * FROM instance_info LIMIT 20;"
docker exec app-db-1 mariadb -upixelfed -p'eXseRRmQGN-FagQfJSG1z' pixelfed_prod -e "SELECT * FROM configs WHERE key_name LIKE '%url%' OR key_name LIKE '%domain%';"