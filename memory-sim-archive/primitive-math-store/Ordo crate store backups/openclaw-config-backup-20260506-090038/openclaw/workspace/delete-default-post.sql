DELETE FROM posts_authors WHERE post_id = (SELECT id FROM posts WHERE slug = 'coming-soon');
DELETE FROM posts_tags WHERE post_id = (SELECT id FROM posts WHERE slug = 'coming-soon');
DELETE FROM posts WHERE slug = 'coming-soon';