-- Ghost built-in SEO settings
UPDATE settings SET value = 'true' WHERE `key` = 'meta_description';
UPDATE settings SET value = 'true' WHERE `key` = 'twitter';
UPDATE settings SET value = 'true' WHERE `key` = 'facebook';
UPDATE settings SET value = 'true' WHERE `key` = 'structured_data';
UPDATE settings SET value = 'true' WHERE `key` = 'schema';
UPDATE settings SET value = 'true' WHERE `key` = 'amp';
UPDATE settings SET value = 'true' WHERE `key` = 'rss';

-- Set meta description for the site
UPDATE settings SET value = 'Warped Reality by Lucerna Media — stories, ideas, and perspectives that bend the narrative.' WHERE `key` = 'description';

-- Set social URLs
UPDATE settings SET value = 'https://bsky.app/profile/warped-reality.bsky.social' WHERE `key` = 'twitter_url';
UPDATE settings SET value = 'https://mastodon.social/@Warped_Reality' WHERE `key` = 'facebook_url';

-- Verify current SEO-related settings
SELECT `key`, value FROM settings WHERE `key` IN ('meta_description', 'twitter', 'facebook', 'structured_data', 'amp', 'rss', 'description', 'twitter_url', 'facebook_url', 'og_image', 'og_title', 'og_description');