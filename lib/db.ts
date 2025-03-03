import sqlite3 from 'sqlite3';
import { open, Database } from 'sqlite';
import path from 'path';
import os from 'os';

class DBManager {
  private static instance: DBManager;
  private db!: Database;

  private constructor() {}

  public static async getInstance(): Promise<DBManager> {
    if (!DBManager.instance) {
      DBManager.instance = new DBManager();
      await DBManager.instance.connect();
      await DBManager.instance.setup(); // Ensure setup is done before returning
    }
    return DBManager.instance;
  }

  private async connect() {
    const dbPath = this.getDbPath();
    console.log('DB path:', dbPath);

    this.db = await open({
      filename: dbPath,
      driver: sqlite3.Database,
    });
  }

  private getDbPath(): string {
    const platform = os.platform();

    // Use platform-specific paths for SQLite database
    if (platform === 'win32') {
      // For Windows, use %APPDATA% or local AppData
      return path.join(process.env.APPDATA || path.join(process.env.LOCALAPPDATA || path.resolve()), 'database.sqlite');
    } else if (platform === 'linux') {
      // For Linux, use /var/lib (or you can use a user-specific path like ~/.local/share/)
      return path.join('/var/lib', 'database.sqlite');
    } else {
      // For other platforms, default to the current directory (or choose a better path)
      return path.join(path.resolve(), 'database.sqlite');
    }
  }

  public get() {
    if (!this.db) {
      throw new Error('Connection not initialized. Call getInstance() first.');
    }
    return this.db;
  }

  public async close() {
    if (this.db) {
      await this.db.close();
      console.log('Connection closed.');
    }
  }

  public async setup() {
    await this.get().exec(`
      CREATE TABLE IF NOT EXISTS files (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        content TEXT NOT NULL,
        access_time DATETIME DEFAULT CURRENT_TIMESTAMP,
        modify_time DATETIME DEFAULT CURRENT_TIMESTAMP
      );

      CREATE TABLE IF NOT EXISTS relation (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        src INTEGER NOT NULL,
        dst INTEGER NOT NULL,
        FOREIGN KEY(src) REFERENCES files(id) ON DELETE CASCADE,
        FOREIGN KEY(dst) REFERENCES files(id) ON DELETE CASCADE
      );

      CREATE TABLE IF NOT EXISTS api_keys (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        key_hash TEXT NOT NULL UNIQUE,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      );
    `);
    console.log('Database setup complete.');
  }
  public async clearAllData() {
    try {
      await this.get().exec('DELETE FROM files');
      await this.get().exec('DELETE FROM relation');
      await this.get().exec('DELETE FROM api_keys');
      console.log('All data cleared from tables.');
    } catch (error) {
      console.error('Error clearing data:', error);
    }
  } 
}

export default DBManager;
