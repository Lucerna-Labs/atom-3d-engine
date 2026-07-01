#!/bin/bash
cd /opt/app
WP="docker compose exec -T wordpress php /var/www/html/wp-cli.phar --path=/var/www/html --allow-root --url=https://lucernamedia.com"

# Create nav menu
MENU_ID=$($WP menu create "Main Navigation" 2>/dev/null | grep -oP '\d+' | tail -1)

if [ -z "$MENU_ID" ]; then
  # Menu might already exist, find it
  MENU_ID=$($WP menu list --fields=term_id,name --format=csv 2>/dev/null | grep "Main Navigation" | cut -d',' -f1)
fi

if [ -z "$MENU_ID" ]; then
  echo "Could not create or find menu"
  exit 1
fi

echo "Menu ID: $MENU_ID"

# Add menu items
$WP menu item add-post-type $MENU_ID --title="Home" --url="/" --type=custom 2>/dev/null
$WP menu item add-post-type $MENU_ID --title="Blog" --object=page --object-id=13 2>/dev/null
$WP menu item add-post-type $MENU_ID --title="About" --url="#about" --type=custom 2>/dev/null

# Assign to navigation location
$WP menu location assign $MENU_ID navigation 2>/dev/null || echo "Location assign may need different location slug"

echo "Done setting up menu"