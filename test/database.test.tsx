import { describe, it, vi, beforeEach, afterEach, expect } from 'vitest'
import NoteController from '@/lib/controller/NoteController'
import EdgeController from '@/lib/controller/EdgeController'
import ChatHistoryController from '@/lib/controller/ChatHistoryController'
import KeyController from '@/lib/controller/KeyController'
import Database from '@/lib/Database'

describe('Edge operations', () => {
  let edgeController: EdgeController

  beforeEach(async () => {
    vi.clearAllMocks()

    const sqliteMock = await import('@tauri-apps/plugin-sql')
    const driverMock = {
      execute: vi.fn().mockResolvedValue({ rowsAffected: 1 }),
      select: vi.fn().mockResolvedValue([])
    }
    sqliteMock.default.load = vi.fn().mockResolvedValue(driverMock)
    edgeController = new EdgeController(':memory:')
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  it('should use SQL constraint to prevent duplicate edges', async () => {
    const executeSpy = vi.spyOn(edgeController as any, 'execute')
    await edgeController.create({ src: 1, dst: 2 })
    expect(executeSpy).toHaveBeenCalledWith(
      expect.stringContaining('ON CONFLICT(src, dst) DO NOTHING'),
      [1, 2]
    )
  })
})

describe('Chat History operations', () => {
  let chatHistoryController: ChatHistoryController

  beforeEach(async () => {
    vi.clearAllMocks()
    const sqliteMock = await import('@tauri-apps/plugin-sql')
    const driverMock = {
      execute: vi.fn().mockResolvedValue({ rowsAffected: 1 }),
      select: vi.fn().mockResolvedValue([])
    }
    sqliteMock.default.load = vi.fn().mockResolvedValue(driverMock)
    chatHistoryController = new ChatHistoryController(':memory:')
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  it('should query chat history in ascending order of timestamp', async () => {
    const selectSpy = vi.spyOn(chatHistoryController as any, 'select')
    await chatHistoryController.readAll()
    expect(selectSpy).toHaveBeenCalledWith(
      expect.stringContaining(`ORDER BY time;`))
  })
})

describe('Database operations', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  it('should create database tables on startup', async () => {
    const database = new Database(':memory:')

    const sqliteMock = await import('@tauri-apps/plugin-sql')
    const driverMock = {
      execute: vi.fn().mockResolvedValue({ rowsAffected: 1 }),
      select: vi.fn().mockResolvedValue([])
    }
    sqliteMock.default.load = vi.fn().mockResolvedValue(driverMock)

    await database.connect()

    expect(driverMock.execute).toHaveBeenCalledWith(expect.stringContaining('CREATE TABLE IF NOT EXISTS Notes'))
    expect(driverMock.execute).toHaveBeenCalledWith(expect.stringContaining('CREATE TABLE IF NOT EXISTS Edges'))
    expect(driverMock.execute).toHaveBeenCalledWith(expect.stringContaining('CREATE TABLE IF NOT EXISTS Keys'))
    expect(driverMock.execute).toHaveBeenCalledWith(expect.stringContaining('CREATE TABLE IF NOT EXISTS Chat_History'))
    expect(driverMock.execute).toHaveBeenCalledWith(expect.stringContaining('CREATE VIRTUAL TABLE IF NOT EXISTS Search'))
  })

  it('should throw error if database is not connected before executing queries', async () => {
    const database = new Database(':memory:')
    await expect(database.execute('SELECT * FROM Notes')).rejects.toThrow('database is disconnected')
    await expect(database.select('SELECT * FROM Notes')).rejects.toThrow('database is disconnected')
  })
})

describe('Note operations', () => {
  let noteController: NoteController

  beforeEach(() => {
    noteController = new NoteController(':memory:')
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  it('should query search with proper SQL for content searching', async () => {
    const selectSpy = vi.spyOn(noteController as any, 'select')
    const searchTerm = 'search term'
    await noteController.search(searchTerm)

    expect(selectSpy).toHaveBeenCalledWith(
      expect.stringContaining('FROM Search'),
      expect.arrayContaining([expect.stringContaining('search term')])
    )

    expect(selectSpy).toHaveBeenCalledWith(
      expect.stringContaining('Search.content MATCH ?'),
      expect.any(Array)
    )
  })
})

describe('Controller connection behavior', () => {
  let noteController: NoteController
  let edgeController: EdgeController
  let chatHistoryController: ChatHistoryController
  let keyController: KeyController

  beforeEach(async () => {
    vi.clearAllMocks()

    const sqliteMock = await import('@tauri-apps/plugin-sql')
    const driverMock = {
      execute: vi.fn().mockResolvedValue({ rowsAffected: 1 }),
      select: vi.fn().mockResolvedValue([])
    }
    sqliteMock.default.load = vi.fn().mockResolvedValue(driverMock)

    noteController = new NoteController(':memory:')
    edgeController = new EdgeController(':memory:')
    chatHistoryController = new ChatHistoryController(':memory:')
    keyController = new KeyController(':memory:')
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  it('NoteController should establish connection before executing queries', async () => {
    const ensureConnectedSpy = vi.spyOn(noteController as any, 'ensureConnected')
    const connectSpy = vi.spyOn(noteController as any, 'connect')

    // Force controller to think it's not connected
    Object.defineProperty(noteController, 'ready', { value: false, writable: true })

    await noteController.readAll()

    expect(ensureConnectedSpy).toHaveBeenCalledTimes(1)
    expect(connectSpy).toHaveBeenCalledTimes(1)

    // Set ready to true as it would be after first connection
    Object.defineProperty(noteController, 'ready', { value: true, writable: true })
    ensureConnectedSpy.mockClear()
    connectSpy.mockClear()

    await noteController.readAll()

    expect(ensureConnectedSpy).toHaveBeenCalledTimes(1)
    expect(connectSpy).not.toHaveBeenCalled()
  })

  it('EdgeController should establish connection before executing queries', async () => {
    const ensureConnectedSpy = vi.spyOn(edgeController as any, 'ensureConnected')
    const connectSpy = vi.spyOn(edgeController as any, 'connect')

    Object.defineProperty(edgeController, 'ready', { value: false, writable: true })
    await edgeController.readAll()

    expect(ensureConnectedSpy).toHaveBeenCalledTimes(1)
    expect(connectSpy).toHaveBeenCalledTimes(1)

    Object.defineProperty(edgeController, 'ready', { value: true, writable: true })
    ensureConnectedSpy.mockClear()
    connectSpy.mockClear()

    await edgeController.readAll()

    expect(ensureConnectedSpy).toHaveBeenCalledTimes(1)
    expect(connectSpy).not.toHaveBeenCalled()
  })

  it('ChatHistoryController should establish connection before executing queries', async () => {
    const ensureConnectedSpy = vi.spyOn(chatHistoryController as any, 'ensureConnected')
    const connectSpy = vi.spyOn(chatHistoryController as any, 'connect')

    Object.defineProperty(chatHistoryController, 'ready', { value: false, writable: true })

    await chatHistoryController.readAll()

    expect(ensureConnectedSpy).toHaveBeenCalledTimes(1)
    expect(connectSpy).toHaveBeenCalledTimes(1)

    Object.defineProperty(chatHistoryController, 'ready', { value: true, writable: true })
    ensureConnectedSpy.mockClear()
    connectSpy.mockClear()

    await chatHistoryController.readAll()

    expect(ensureConnectedSpy).toHaveBeenCalledTimes(1)
    expect(connectSpy).not.toHaveBeenCalled()
  })

  it('KeyController should establish connection before executing queries', async () => {
    const ensureConnectedSpy = vi.spyOn(keyController as any, 'ensureConnected')
    const connectSpy = vi.spyOn(keyController as any, 'connect')

    Object.defineProperty(keyController, 'ready', { value: false, writable: true })

    await keyController.readAll()

    expect(ensureConnectedSpy).toHaveBeenCalledTimes(1)
    expect(connectSpy).toHaveBeenCalledTimes(1)

    Object.defineProperty(keyController, 'ready', { value: true, writable: true })
    ensureConnectedSpy.mockClear()
    connectSpy.mockClear()

    await keyController.readAll()

    expect(ensureConnectedSpy).toHaveBeenCalledTimes(1)
    expect(connectSpy).not.toHaveBeenCalled()
  })
})
