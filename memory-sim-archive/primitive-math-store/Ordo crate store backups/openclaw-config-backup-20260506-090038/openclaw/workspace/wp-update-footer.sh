#!/bin/bash
cd /opt/app
# Find the footer template part post ID
FOOTER_ID=$(docker compose exec -T wordpress php /var/www/html/wp-cli.phar post list --post_type=wp_template_part --fields=ID,post_title,post_name --post_status=any --path=/var/www/html --allow-root --url=https://lucernamedia.com 2>/dev/null | grep -i footer | awk '{print $1}')

if [ -z "$FOOTER_ID" ]; then
  echo "No footer template part found, creating one..."
  FOOTER_ID=$(docker compose exec -T wordpress php /var/www/html/wp-cli.phar post create --post_type=wp_template_part --post_title='Footer' --post_name='footer' --post_status='publish' --post_content="$(cat /tmp/wp-footer.html)" --path=/var/www/html --allow-root --url=https://lucernamedia.com 2>/dev/null | grep -oP '\d+' | tail -1)
  echo "Created footer with ID: $FOOTER_ID"
else
  echo "Found footer template part ID: $FOOTER_ID"
  docker compose exec -T wordpress php /var/www/html/wp-cli.phar post update $FOOTER_ID --post_content="$(cat /tmp/wp-footer.html)" --path=/var/www/html --allow-root --url=https://lucernamedia.com
fi

echo "Footer update complete"