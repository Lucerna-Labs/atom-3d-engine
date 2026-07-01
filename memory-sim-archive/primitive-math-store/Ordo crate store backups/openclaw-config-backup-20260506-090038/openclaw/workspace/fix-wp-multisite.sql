UPDATE wp_blogs SET domain='lucernamedia.com' WHERE blog_id=1;
UPDATE wp_sitemeta SET meta_value='https://lucernamedia.com' WHERE meta_key='siteurl';
SELECT blog_id, domain FROM wp_blogs;
SELECT meta_key, meta_value FROM wp_sitemeta WHERE meta_key='siteurl';