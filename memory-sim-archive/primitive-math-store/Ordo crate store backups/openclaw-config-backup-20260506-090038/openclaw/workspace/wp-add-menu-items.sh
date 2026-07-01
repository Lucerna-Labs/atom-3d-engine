#!/bin/bash
cd /opt/app
WP="docker compose exec -T wordpress php /var/www/html/wp-cli.phar --path=/var/www/html --allow-root --url=https://lucernamedia.com"

# Add items to menu 3
$WP menu item add-custom 3 "Home" "/"
$WP menu item add-post-type 3 --title="Blog" --object=page --object-id=13
$WP menu item add-custom 3 "About" "/#about"

echo "Menu items added"