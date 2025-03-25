import Database from "@/lib/Database"

// Edge is the TypeScript type for the Edges database schema.
export type Edge = {
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

  // Create a new edge or update an existing one if src and dst
  // already exists. ON CONFLICT action to DO NOTHING since there are
  // only primary keys.
  async create(edge: Edge) : Promise<void> {
    await this.ensureConnected()
    const query = `INSERT INTO Edges (src, dst)
    VALUES (?, ?)
    ON CONFLICT(src, dst) DO NOTHING`
    await this.execute(query, [edge.src, edge.dst])
  }

  // read fetches the edge with the specified src and dst, if it exists.
  async read(src: number, dst: number) : Promise<Edge | null> {
    await this.ensureConnected()
    const query = `SELECT * FROM Edges WHERE src = ? AND dst = ?;`
    const result = await this.select<Edge>(query, [src, dst])
    return result.length > 0 ? result[0] as Edge : null
  }

  // readAll fetches all  from the database.
  async readAll() : Promise<Edge[]> {
    await this.ensureConnected()
    return await this.select<Edge>(`SELECT * FROM Edges;`)
  }

  // delete the edge with the specified src and dst.
  async delete(src: number, dst: number) : Promise<void> {
    await this.ensureConnected()
    const query = `DELETE FROM Edges WHERE src = ? AND dst = ?;`
    await this.execute(query, [src, dst])
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
