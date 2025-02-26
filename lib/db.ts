import sqlite3 from 'sqlite3';
import { open, Database } from 'sqlite';
import path from 'path';

// Open the SQLite database connection
export async function connectDB(): Promise<Database> {
  const dirname = path.resolve();  // Get the absolute path of the current directory
  const dbPath = path.join(dirname, 'database.sqlite');
  
  console.log('Database path:', dbPath);

  return open({
    filename: dbPath,
    driver: sqlite3.Database,
  });
}
