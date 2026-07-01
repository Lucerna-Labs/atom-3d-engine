-- Add subsites to WordPress multisite
-- Site 2: Lucerna Labs
INSERT INTO wp_blogs (site_id, domain, path, registered, last_updated, public, archived, mature, spam, deleted, lang_id)
VALUES (1, 'lucernalabs.lucernamedia.com', '/', NOW(), NOW(), 1, 0, 0, 0, 0, 0);

-- Site 3: Warped Reality
INSERT INTO wp_blogs (site_id, domain, path, registered, last_updated, public, archived, mature, spam, deleted, lang_id)
VALUES (1, 'warpedreality.lucernamedia.com', '/', NOW(), NOW(), 1, 0, 0, 0, 0, 0);

-- Check results
SELECT blog_id, domain, path FROM wp_blogs;