#!/bin/bash
cd /opt/app
WP="docker compose exec -T wordpress php /var/www/html/wp-cli.phar --path=/var/www/html --allow-root --url=https://lucernamedia.com"

# Add Blog page as a post type menu item
$WP menu item add-post-type 3 --title="Blog" --object=page --object-id=13 --type=post_type

echo "Done"