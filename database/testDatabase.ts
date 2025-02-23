import { connectDB } from './db';

async function testDatabase() {
  const db = await connectDB();

  await db.run(`
    CREATE TABLE IF NOT EXISTS files (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      title TEXT NOT NULL,
      content TEXT NOT NULL
    );
  `);

  await db.run(
    `INSERT INTO files (title, content) VALUES (?, ?)`,
    ['Test File', 'This is a test content']
  );

  // Retrieve all files
  const files = await db.all(`SELECT * FROM files`);
  console.log('Files:', files);

  await db.close();
}

testDatabase().catch((err) => {
  console.error('Error:', err);
});
