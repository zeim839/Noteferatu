pub const SCHEMA_VERSION_0: &str = "
CREATE TABLE IF NOT EXISTS File (
  id            INTEGER  PRIMARY KEY AUTOINCREMENT,
  name          TEXT     NOT NULL,
  parent        INTEGER,
  remote_id     TEXT,
  is_deleted    BOOLEAN  NOT NULL,
  created_at    INTEGER  NOT NULL,
  modified_at   INTEGER  NOT NULL,
  synced_at     INTEGER,
  is_folder     BOOLEAN  NOT NULL,
  is_bookmarked BOOLEAN  NOT NULL DEFAULT FALSE,

  FOREIGN KEY (parent) REFERENCES File(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Tag (
  name       TEXT       PRIMARY KEY,
  color      VARCHAR(6) NOT NULL     DEFAULT \"000000\",
  created_at INTEGER    NOT NULL
);

CREATE TABLE IF NOT EXISTS TagBind (
  tag  TEXT,
  file INTEGER,

  FOREIGN KEY (tag) REFERENCES Tag(name)
    ON DELETE CASCADE
    ON UPDATE CASCADE,

  FOREIGN KEY (file) REFERENCES File(id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,

  PRIMARY KEY (tag, file)
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

CREATE TRIGGER prevent_duplicate_filenames_insert
BEFORE INSERT ON File
FOR EACH ROW
BEGIN
  SELECT CASE
    WHEN EXISTS (
      SELECT 1 FROM File
      WHERE name = NEW.name
        AND parent IS NEW.parent
        AND is_deleted = 0
    )
    THEN RAISE(ABORT, 'A file with this name already exists in the directory')
  END;
END;

CREATE TRIGGER prevent_duplicate_filenames_update
BEFORE UPDATE OF name, parent ON File
FOR EACH ROW
BEGIN
  SELECT CASE
    WHEN EXISTS (
      SELECT 1 FROM File
      WHERE name = NEW.name
        AND parent IS NEW.parent
        AND id != NEW.id
        AND is_deleted = 0
    )
    THEN RAISE(ABORT, 'A file with this name already exists in the directory')
  END;
END;
";
