import { connectDB } from './db';

async function setupDatabase() {
  const db = await connectDB();

  await db.exec(`
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
  await db.close();
}

setupDatabase().catch((err) => {
  console.error('Database setup failed:', err);
});
