import Database from "@/lib/Database"

// KeysController is the TypeScript type for the keys database schema.
export type Keys = {
  id?        : number
  key_hash   : string
  created_at : number
}

// KeyController manages keys in the database.
class KeyController extends Database {

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

  // create a new key or update an existing one if ID already exists.
  async create(key: Keys) : Promise<void> {
    await this.ensureConnected()
    const query = `INSERT INTO Keys (id, key_hash, created_at)
    VALUES (?, ?, ?)
    ON CONFLICT(id) DO UPDATE SET
    key_hash = excluded.key_hash,
    created_at = excluded.created_at;`
    await this.execute(query, [key.id, key.key_hash, key.created_at])
  }

  // read fetches the key with the specified ID, if it exists.
  async read(id: number) : Promise<Keys | null> {
    await this.ensureConnected()
    const query = `SELECT * FROM Keys WHERE id = ?;`
    const result = await this.select<Keys>(query, [id])
    return result.length > 0 ? result[0] as Keys : null
  }

  // Add
  async readAll(): Promise<Keys[]> {
    await this.ensureConnected()
    const query = `SELECT * FROM Keys;`
    const result = await this.select<Keys>(query)
    return result.length ? result : []
}

  // delete the key with the specified ID.
  async delete(id: number) : Promise<void> {
    await this.ensureConnected()
    const query = `DELETE FROM Keys WHERE id = ?;`
    await this.execute(query, [id])
  }

  // deleteAll removes all records in the Keys tables.
  async deleteAll(): Promise<void> {
    await this.ensureConnected()
    await this.execute(`DELETE FROM Keys;`)
  }

  // count returns the number of records in the Keys table.
  async count(): Promise<number> {
    await this.ensureConnected()
    const query = `SELECT COUNT(*) FROM Keys;`
    const result = await this.select<{'COUNT(*)': number}>(query)
    return result[0]['COUNT(*)']
  }
}

export default KeyController
