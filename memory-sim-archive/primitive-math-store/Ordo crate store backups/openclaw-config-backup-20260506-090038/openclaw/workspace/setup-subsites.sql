-- Create options tables for subsites
CREATE TABLE IF NOT EXISTS wp_2_options LIKE wp_options;
CREATE TABLE IF NOT EXISTS wp_3_options LIKE wp_options;

-- Set site titles and URLs
INSERT INTO wp_2_options (option_name, option_value, autoload) VALUES
('siteurl', 'https://lucernalabs.lucernamedia.com', 'yes'),
('home', 'https://lucernalabs.lucernamedia.com', 'yes'),
('blogname', 'Lucerna Labs', 'yes'),
('blogdescription', 'Creative tech by Lucerna Media', 'yes'),
('blog_public', '1', 'yes');

INSERT INTO wp_3_options (option_name, option_value, autoload) VALUES
('siteurl', 'https://warpedreality.lucernamedia.com', 'yes'),
('home', 'https://warpedreality.lucernamedia.com', 'yes'),
('blogname', 'Warped Reality', 'yes'),
('blogdescription', 'by Lucerna Media', 'yes'),
('blog_public', '1', 'yes');

SELECT 'Site 2 options:', COUNT(*) FROM wp_2_options;
SELECT 'Site 3 options:', COUNT(*) FROM wp_3_options;