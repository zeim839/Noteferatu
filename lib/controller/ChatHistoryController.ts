import Database from "@/lib/Database"

// Chat_History is the TypeScript type for the Chat_History database
// schema.
export type Chat_History = {
  id?        : number
  role       : 'assistant' | 'user' | 'tool'
  tool_name? : string,
  content    : string
  time       : number
}

// Controller manages chat_history in the database.
class ChatHistoryController extends Database {

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

  // create a new chat_history or update an existing one if ID already exists.
  async create(chat_history: Chat_History) : Promise<void> {
    await this.ensureConnected()
    const query = `INSERT INTO Chat_History (id, role, tool_name, content, time)
    VALUES (?, ?, ?, ?, ?)
    ON CONFLICT(id) DO UPDATE SET
    role = excluded.role,
    tool_name = excluded.tool_name,
    content = excluded.content,
    time = excluded.time;`
    await this.execute(query, [chat_history.id, chat_history.role,
      chat_history.tool_name, chat_history.content, chat_history.time])
  }

  // read in all chats by time ascending.
  async readAll() : Promise<Chat_History[]> {
    await this.ensureConnected()
    return await this.select<Chat_History>(`SELECT * FROM Chat_History ORDER BY time;`)
  }

  // delete the chat_history with the specified ID.
  async delete(id: number) : Promise<void> {
    await this.ensureConnected()
    const query = `DELETE FROM Chat_History WHERE id = ?;`
    await this.execute(query, [id])
  }

  // deleteAll removes all records in the Chat_History tables.
  async deleteAll(): Promise<void> {
    await this.ensureConnected()
    await this.execute(`DELETE FROM Chat_History;`)
  }

  // count returns the number of records in the Chat_History table.
  async count(): Promise<number> {
    await this.ensureConnected()
    const query = `SELECT COUNT(*) FROM Chat_History;`
    const result = await this.select<{'COUNT(*)': number}>(query)
    return result[0]['COUNT(*)']
  }
}

export default ChatHistoryController
