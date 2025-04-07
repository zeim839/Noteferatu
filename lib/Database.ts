import sqlite from '@tauri-apps/plugin-sql'

// SETUP_QUERY is an SQL statement that sets up the database schema.
const SETUP_QUERY = `
CREATE TABLE IF NOT EXISTS Notes (
  id      INTEGER  PRIMARY KEY AUTOINCREMENT,
  title   TEXT     NOT NULL,
  content TEXT     NOT NULL,
  atime   INTEGER,
  mtime   INTEGER
);

CREATE TABLE IF NOT EXISTS Edges (
  src INTEGER NOT NULL,
  dst INTEGER NOT NULL,

  FOREIGN KEY (src) REFERENCES Notes(id) ON DELETE CASCADE,
  FOREIGN KEY (dst) REFERENCES Notes(id) ON DELETE CASCADE,
  PRIMARY KEY (src, dst)
);

CREATE TABLE IF NOT EXISTS Keys (
  id         INTEGER  PRIMARY KEY AUTOINCREMENT,
  key_hash   TEXT     NOT NULL UNIQUE,
  created_at INTEGER
);

CREATE TABLE IF NOT EXISTS Chat_History (
  id        INTEGER  PRIMARY KEY AUTOINCREMENT,
  role      TEXT     NOT     NULL,
  tool_name TEXT     DEFAULT NULL,
  content   TEXT     NOT     NULL,
  time      INTEGER
);

CREATE VIRTUAL TABLE IF NOT EXISTS Search USING fts5(
  id, title, content, tokenize = "unicode61 tokenchars '!@#$%^&*()_-+=<>,.?~''\"\"'"
);

CREATE TRIGGER IF NOT EXISTS insert_search
  AFTER INSERT ON Notes
  BEGIN
    INSERT INTO Search (id, title, content)
    VALUES (NEW.id, NEW.title, NEW.content);
  END;

CREATE TRIGGER IF NOT EXISTS update_search
  AFTER UPDATE ON Notes
  BEGIN
    UPDATE Search
    SET
      title   = NEW.title,
      content = NEW.content
    WHERE id = NEW.id;
  END;

CREATE TRIGGER IF NOT EXISTS delete_search
  AFTER DELETE ON Notes
  BEGIN
    DELETE FROM Search
    WHERE id = OLD.id;
  END;
`

// Database wraps the Tauri SQLite database API into a class. It can be
// used to interface with the database directly, or as a superclass for
// ORMs.
class Database {

  private driver : sqlite | null = null
  private path : string = ""

  // Creates a new database object which will connect to the SQLite
  // instance at the given path, creating a new SQLite database if
  // a file is not found at the path.
  constructor(path: string) {
    this.path = `sqlite:${path}`
  }

  // connect to the database by opening the SQLite file path specified
  // in the constructor and creating necessary tables (if they don't
  // already exist).
  async connect() {
    this.driver = await sqlite.load(this.path)
    await this.driver.execute(SETUP_QUERY)
  }

  // Executes an SQL expression. Throws an error if the database
  // instance has not been connected.
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  async execute(query: string, bindValues?: unknown[]) : Promise<any> {
    if (!this.driver) {
      throw new Error("cannot execute query while database is disconnected")
    }
    return await this.driver.execute(query, bindValues)
  }

  // Passes in a SELECT query to the database for execution. An error is
  // throw if the database instance has not been connected.
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  async select<T = any>(query: string, bindValues?: unknown[]) : Promise<T[]> {
    if (!this.driver) {
      throw new Error("cannot execute select query while database is disconnected")
    }
    const result = await this.driver.select(query, bindValues)
    if (!Array.isArray(result)) {
      throw new Error("Unexpected result format: expected an array")
    }
    return result as T[]
  }
}

export default Database
