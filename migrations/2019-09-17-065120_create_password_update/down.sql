-- This file should undo anything in `up.sql`
DROP TABLE `password_updates`;
DELETE EVENT `password_updates_cleaner_event`;