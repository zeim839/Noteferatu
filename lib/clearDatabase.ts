import { connectDB } from './db';

async function clearDatabase() {
  const db = await connectDB();

  await db.run('DELETE FROM files');
  await db.run('DELETE FROM relation');
  await db.run('DELETE FROM api_keys');

  console.log('All records have been cleared from the tables.');

  await db.run('DELETE FROM sqlite_sequence WHERE name="files"');
  await db.run('DELETE FROM sqlite_sequence WHERE name="relation"');
  await db.run('DELETE FROM sqlite_sequence WHERE name="api_keys"');

  console.log('Auto-increment ids have been reset.');

  await db.close();
}

clearDatabase().catch((err) => {
  console.error('Failed to clear database:', err);
});
