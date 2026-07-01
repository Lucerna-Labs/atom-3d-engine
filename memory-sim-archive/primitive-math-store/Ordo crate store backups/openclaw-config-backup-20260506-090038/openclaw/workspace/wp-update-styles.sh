#!/bin/bash
cd /opt/app
CONTENT=$(cat /tmp/wp-global-styles.json | tr -d '\n')
docker compose exec -T wordpress php /var/www/html/wp-cli.phar post update 10 --post_content="$CONTENT" --path=/var/www/html --allow-root --url=https://lucernamedia.com