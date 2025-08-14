/// [Manager](super::Manager) SQL schema.
pub const SCHEMA_VERSION_0: &str = "
CREATE TABLE IF NOT EXISTS Conversation (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  name        TEXT    NOT NULL,
  created_at  INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS Message (
  id      INTEGER NOT NULL,
  conv_id INTEGER NOT NULL,
  object  TEXT    NOT NULL,

  PRIMARY KEY (id, conv_id),
  FOREIGN KEY (conv_id) REFERENCES Conversation(id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);
";
