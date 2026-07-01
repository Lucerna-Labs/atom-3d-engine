#!/bin/bash
cd /opt/app
CONTENT=$(cat /tmp/wp-homepage.html)
docker compose exec -T wordpress php /var/www/html/wp-cli.phar post update 9 --post_title='Home' --post_name='home' --post_content="$CONTENT" --path=/var/www/html --allow-root --url=https://lucernamedia.com