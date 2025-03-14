import Database from "@/lib/Database"

// Note is the TypeScript type for the Notes database schema.
export type Note = {
  id?      : number
  title   : string
  content : string
  atime   : number
  mtime   : number
}

// NoteController manages notes in the database.
class NoteController extends Database {

  // ready tracks whether the controller is ready to start executing
  // transactions. It is true when the database is successfully connected.
  private ready : boolean = false

  constructor(path: string) {
    super(path)
    this.connect()
      .then(() => { this.ready = true })
      .catch(() => { this.ready = false })
  }

  // Ensure that the database is connected. If not, then try to connect.
  private async ensureConnected() {
    if (this.ready) {
      return
    }
    await this.connect()
    this.ready = true
  }

  // create a new note or update an existing one if ID already exists.
  async create(note: Note) : Promise<void> {
    await this.ensureConnected()
    const query = `INSERT INTO Notes (id, title, content, atime, mtime)
    VALUES (?, ?, ?, ?, ?)
    ON CONFLICT(id) DO UPDATE SET
    title = excluded.title,
    content = excluded.content,
    atime = excluded.atime,
    mtime = excluded.mtime;`
    await this.execute(query, [note.id, note.title, note.content, note.atime, note.mtime])
  }

  // read fetches the note with the specified ID, if it exists.
  async read(id: number) : Promise<Note | null> {
    await this.ensureConnected()
    const query = `SELECT * FROM Notes WHERE id = ?;`
    const result = await this.select<Note>(query, [id])
    return result.length > 0 ? result[0] as Note : null
  }

  // readAll fetches all notes from the database.
  async readAll() : Promise<Note[]> {
    await this.ensureConnected()
    return await this.select<Note>(`SELECT * FROM Notes;`)
  }

  async getRecents(count: number) : Promise<Note[]> {
    await this.ensureConnected()
    const query = `SELECT * FROM Notes ORDER BY atime DESC LIMIT ?;`
    return await this.select<Note>(query, [count])
  }

  // delete the note with the specified ID.
  async delete(id: number) : Promise<void> {
    await this.ensureConnected()
    const query = `DELETE FROM Notes WHERE id = ?;`
    await this.execute(query, [id])
  }

  // deleteAll removes all records in the Notes tables.
  async deleteAll(): Promise<void> {
    await this.ensureConnected()
    await this.execute(`DELETE FROM Notes;`)
  }

  // count returns the number of records in the Notes table.
  async count(): Promise<number> {
    await this.ensureConnected()
    const query = `SELECT COUNT(*) FROM Notes;`
    const result = await this.select<{'COUNT(*)': number}>(query)
    return result[0]['COUNT(*)']
  }
}

export default NoteController
