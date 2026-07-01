-- Update Ghost social settings to Bluesky and Mastodon
-- Ghost's "twitter" field is used for X/Twitter URL, "facebook" for Facebook URL
-- Since Jesse doesn't use either, we'll repurpose or clear them
-- Ghost doesn't have native Bluesky/Mastodon fields, so we'll add them via code injection

-- Clear Twitter and Facebook URLs since they're not used
UPDATE settings SET value = '' WHERE `key` = 'twitter_url';
UPDATE settings SET value = '' WHERE `key` = 'facebook_url';

-- Disable Twitter card meta (we'll handle social via code injection)
UPDATE settings SET value = 'false' WHERE `key` = 'twitter';

-- Verify
SELECT `key`, value FROM settings WHERE `key` IN ('twitter', 'twitter_url', 'facebook_url');