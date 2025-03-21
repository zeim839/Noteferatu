import Database from "@/lib/Database"

// Edge is the TypeScript type for the Edges database schema.
export type Edge = {
  id?  : number
  src  : number
  dst  : number
}

// EdgeController manages edge in the database.
class EdgeController extends Database {

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

  // create a new edge or update an existing one if ID already exists.
  async create(edge: Edge) : Promise<void> {
    await this.ensureConnected()
    const query = `INSERT INTO Edges (id,src,dst)
    VALUES (?, ?, ?)
    ON CONFLICT(id) DO UPDATE SET
    id  = excluded.id,
    src = excluded.src,
    dst = excluded.dst;`
    await this.execute(query, [edge.id, edge.src, edge.dst])
  }

  // read fetches the edge with the specified ID, if it exists.
  async read(id: number) : Promise<Edge | null> {
    await this.ensureConnected()
    const query = `SELECT * FROM Edges WHERE id = ?;`
    const result = await this.select<Edge>(query, [id])
    return result.length > 0 ? result[0] as Edge : null
  }

  // readAll fetches all  from the database.
  async readAll() : Promise<Edge[]> {
    await this.ensureConnected()
    return await this.select<Edge>(`SELECT * FROM Edges;`)
  }

  // delete the edge with the specified ID.
  async delete(id: number) : Promise<void> {
    await this.ensureConnected()
    const query = `DELETE FROM Edges WHERE id = ?;`
    await this.execute(query, [id])
  }

  // count returns the number of records in the Edges table.
  async count() : Promise<number> {
    await this.ensureConnected()
    const query = `SELECT COUNT(*) FROM Edges;`
    const result = await this.select<{'COUNT(*)': number}>(query)
    return result[0]['COUNT(*)']
  }
}

export default EdgeController
