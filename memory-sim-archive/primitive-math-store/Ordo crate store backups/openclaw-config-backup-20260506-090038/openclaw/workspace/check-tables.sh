#!/bin/bash
cd /opt/app
docker exec app-db-1 mariadb -upixelfed -p'eXseRRmQGN-FagQfJSG1z' pixelfed_prod -e "SHOW TABLES;"