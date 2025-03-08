import Database from './Database'

export async function getNotesData(db: Database, queryAmount: number) {
    const query = "SELECT * FROM Notes ORDER BY atime DESC LIMIT ?"; 
    return await db.select(query,[queryAmount])
  }
  
  export async function createNote(db: Database, title: string, content: string){
    const query = "INSERT INTO Notes (title, content) VALUES (?, ?)"
    return await db.execute(query, [title, content])
  }
