UPDATE wp_options SET option_value='https://lucernamedia.com' WHERE option_name='siteurl';
UPDATE wp_options SET option_value='https://lucernamedia.com' WHERE option_name='home';
SELECT option_name, option_value FROM wp_options WHERE option_name IN ('siteurl','home');