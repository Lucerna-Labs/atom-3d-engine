UPDATE settings SET value='Warped Reality' WHERE `key`='title';
UPDATE settings SET value='by Lucerna Media' WHERE `key`='description';
SELECT `key`, value FROM settings WHERE `key` IN ('title', 'description');