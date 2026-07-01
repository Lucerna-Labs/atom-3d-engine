#!/bin/bash
docker exec app-wordpress-1 sed -i "s|define('WP_SITEURL', 'https://' . \$_SERVER\['HTTP_HOST'\]);|define('WP_SITEURL', 'https://lucernamedia.com');|" /var/www/html/wp-config.php
docker exec app-wordpress-1 sed -i "s|define('WP_HOME', 'https://' . \$_SERVER\['HTTP_HOST'\]);|define('WP_HOME', 'https://lucernamedia.com');|" /var/www/html/wp-config.php
echo "wp-config updated"