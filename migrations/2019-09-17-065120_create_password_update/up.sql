CREATE TABLE IF NOT EXISTS `password_updates` (
  `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `user_id` INT NOT NULL,
  `token` VARCHAR(64) NOT NULL,
  `date_created` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=1;

CREATE EVENT IF NOT EXISTS `password_updates_cleaner_event`
ON SCHEDULE
  EVERY 1 DAY_HOUR
  COMMENT 'Clean up password change (password reset) token daily'
  DO
    DELETE FROM `password_updates` WHERE `date_created` > DATE_SUB(NOW(), INTERVAL 24 HOUR); 