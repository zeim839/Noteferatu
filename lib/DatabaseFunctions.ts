import Database from './Database'

  export async function getNotesData(db: Database, queryAmount: number) {
    const query = "SELECT * FROM Notes ORDER BY atime DESC LIMIT ?"; 
    return await db.select(query,[queryAmount])
  }

// Make sure to always convert time to seconds Date.now() returns milliseconds
  export async function createNote(db: Database, title: string, content: string) {
    const currentTime = Math.floor(Date.now()/1000);
    const query = "INSERT INTO Notes (title, content, atime, mtime) VALUES (?,?,?,?)"
    return await db.execute(query, [title, content,currentTime,currentTime])
  }

  export async function deleteNote(db: Database) {
    const query = "DELETE FROM NOTES"
    return await db.execute(query)
  }
