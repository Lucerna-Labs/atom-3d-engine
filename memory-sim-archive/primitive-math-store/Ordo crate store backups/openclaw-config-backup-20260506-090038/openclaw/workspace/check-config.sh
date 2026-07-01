#!/bin/bash
cd /opt/app
docker exec app-db-1 mariadb -upixelfed -p'eXseRRmQGN-FagQfJSG1z' pixelfed_prod -e "DESCRIBE config_cache;"
docker exec app-db-1 mariadb -upixelfed -p'eXseRRmQGN-FagQfJSG1z' pixelfed_prod -e "SELECT * FROM config_cache LIMIT 10;"