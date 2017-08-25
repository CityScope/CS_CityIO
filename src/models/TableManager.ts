import * as deepEqual from 'deep-equal'
import { waitDuration } from '../config/constants'

export interface ITable {
  id: string
  timestamp: number
  error: string
  grid: any
  objects: any
}

function compareTableContents (table1: ITable, table2: ITable): boolean {
  const contents1: any = {grid: table1.grid, objects: table1.objects}
  const contents2: any = {grid: table2.grid, objects: table2.objects}
  return deepEqual(contents1, contents2)
}

export interface ITables {
  [id: string]: ITable
}

export const emptyTable: ITable = {
  error: '',
  grid: [],
  id: '',
  objects: [],
  timestamp: 0,
}

export default class TableManager {

  public static async loadTables (): Promise<TableManager> {
      return new TableManager({})
  }

  private tables: ITables
  private isLocal: boolean

  constructor (tables: {[id: string]: ITable}) {
    this.tables = tables // will be empty
  }

  public getList () {
    return Object.keys(this.tables)
  }

  public async getTable (tableName: string): Promise<ITable> {
    if (tableName in this.tables) {
      return Promise.resolve(this.tables[tableName])
    } else {
      return Promise.resolve({...emptyTable,
        error: 'table data not found (current version does not look into firebase Database)',
      })
    }
  }

  public async clearTable (tableName): Promise<void> {
    delete this.tables[tableName]
  }

  // overwrites
  public async addTable (tableName: string, tableData: ITable): Promise<ITable> {
    // tableData will not contain id.
    this.tables[tableName] = tableData
    return Promise.resolve(tableData)
  }
}
