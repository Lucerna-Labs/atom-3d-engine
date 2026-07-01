#!/bin/bash
docker exec app-wordpress-1 sed -i "s/lucernamedia-u70513.vm.elestio.app/lucernamedia.com/" /var/www/html/wp-config.php
docker exec app-wordpress-1 grep DOMAIN_CURRENT_SITE /var/www/html/wp-config.php
echo "Done"