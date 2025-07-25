pub const SCHEMA_VERSION_0: &str = "
CREATE TABLE IF NOT EXISTS File (
  id          INTEGER  PRIMARY KEY AUTOINCREMENT,
  name        TEXT     NOT NULL,
  parent      INTEGER,
  remote_id   TEXT,
  is_deleted  BOOLEAN  NOT NULL,
  created_at  INTEGER  NOT NULL,
  modified_at INTEGER  NOT NULL,
  synced_at   INTEGER,
  is_folder   BOOLEAN  NOT NULL,

  FOREIGN KEY (parent) REFERENCES File(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);

CREATE TRIGGER enforce_parent_is_folder
BEFORE INSERT ON File
FOR EACH ROW
WHEN NEW.parent IS NOT NULL
BEGIN
  SELECT CASE
    WHEN (SELECT is_folder FROM File WHERE id = NEW.parent) = 0
    THEN RAISE(ABORT, 'Parent must be a folder')
    WHEN (SELECT is_deleted FROM File WHERE id = NEW.parent) = 1
    THEN RAISE(ABORT, 'Parent must not be deleted')
  END;
END;

CREATE TRIGGER enforce_parent_is_folder_update
BEFORE UPDATE OF parent ON File
FOR EACH ROW
WHEN NEW.parent IS NOT NULL
BEGIN
  SELECT CASE
    WHEN (SELECT is_folder FROM File WHERE id = NEW.parent) = 0
    THEN RAISE(ABORT, 'Parent must be a folder')
    WHEN (SELECT is_deleted FROM File WHERE id = NEW.parent) = 1
    THEN RAISE(ABORT, 'Parent must not be deleted')
  END;
END;
";
