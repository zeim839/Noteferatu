import DBManager from './db';

async function useDatabase() {
  try {
    const manager = await DBManager.getInstance();
    const db = manager.get();

    await db.run(`INSERT INTO files (title, content) VALUES (?, ?)`, [
      'My First File',
      'This is the content of the file.',
    ]);

    console.log('File inserted successfully');
  } catch (error) {
    console.error('Database operation failed:', error);
  }
}

async function fetchFiles() {
  try {
    const manager = await DBManager.getInstance();
    const db = manager.get();

    const files = await db.all(`SELECT * FROM files`);
    
    console.log('Fetched Files:', files);
    return files;
  } catch (error) {
    console.error('Database query failed:', error);
    return [];
  }
}

async function run() {
  await useDatabase();
  await fetchFiles();
}

run();
