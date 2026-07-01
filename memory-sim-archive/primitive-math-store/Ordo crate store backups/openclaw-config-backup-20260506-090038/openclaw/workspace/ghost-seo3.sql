-- Set OG/social meta for SEO
UPDATE settings SET value = 'Warped Reality' WHERE `key` = 'og_title';
UPDATE settings SET value = 'Warped Reality by Lucerna Media — stories, ideas, and perspectives that bend the narrative.' WHERE `key` = 'og_description';
UPDATE settings SET value = 'Warped Reality' WHERE `key` = 'twitter_title';
UPDATE settings SET value = 'Warped Reality by Lucerna Media — stories, ideas, and perspectives that bend the narrative.' WHERE `key` = 'twitter_description';

-- Set accent color to white for dark theme (buttons, links, etc.)
UPDATE settings SET value = '#ffffff' WHERE `key` = 'accent_color';

-- Remove the default cover image since it's bright
UPDATE settings SET value = '' WHERE `key` = 'cover_image';

-- Verify
SELECT `key`, value FROM settings WHERE `key` IN ('og_title', 'og_description', 'og_image', 'twitter_title', 'twitter_description', 'twitter_image', 'accent_color', 'cover_image');