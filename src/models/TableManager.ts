// import { fromJS, Map } from 'immutable'
import * as deepEqual from 'deep-equal'
import { database } from 'firebase-admin'
import { waitDuration } from '../config/constants'
import {
  connectToFirebase,
  createOrUpdateTable,
  dropTable,
  getLatestTable,
  getLatestTables,
  isTableRegistered,
  updateTimeStamp } from '../helpers/api'

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

  public static async loadTables (isLocal: boolean = false): Promise<TableManager> {
    if ( !isLocal ) {
      const tables: ITables = await getLatestTables()
      return new TableManager(tables)
    } else {
      return new TableManager({}, true)
    }
  }

  private tables: ITables
  private isLocal: boolean
  private ref: database.Reference

  constructor (tables: {[id: string]: ITable}, isLocal: boolean = false) {

    this.isLocal = isLocal

    if (!this.isLocal) {
      this.ref = connectToFirebase ()

      if (this.ref === null) {
        this.isLocal = true
      }

    } else {
      console.log ('GOING LOCAL')
    }

    this.tables = tables

  }

  public getList () {
    return Object.keys(this.tables)
  }

  public getRef () {
    return this.ref
  }

  public async getTable (tableName: string): Promise<ITable> {
    if (tableName in this.tables) {
      return Promise.resolve(this.tables[tableName])
    } else {
      if (this.isLocal) {
        return Promise.resolve({...emptyTable, error: 'local mode; will not query DB'})
      }
      // look inside the DB
      const tableExist = await isTableRegistered(tableName)
      if (tableExist) {
        const newTable: ITable = await getLatestTable(tableName)
        this.tables[tableName] = newTable

        return newTable
      } else {
        return Promise.resolve({...emptyTable, error: 'table data not found'})
      }
    }
  }

  public async clearTable (tableName): Promise<void> {
    delete this.tables[tableName]
    if (!this.isLocal) {
      await dropTable(tableName)
    }
  }

  // overwrites
  public async addTable (tableName: string, tableData: ITable): Promise<ITable> {

    if (this.isLocal) {
      const newTable: ITable = {...tableData, timestamp: Date.now(), id: 'local'}
      this.tables[tableName] = newTable

      return newTable
    }

    if (tableName in this.tables) {
      if (tableData.timestamp - this.tables[tableName].timestamp > waitDuration) {

        if (compareTableContents(this.tables[tableName], tableData)) {
          if (!this.isLocal) {
            await updateTimeStamp(tableName, this.tables[tableName].id, tableData.timestamp)
          }
          return {...this.tables[tableName], timestamp: tableData.timestamp}

        } else {
          let tableWithId: ITable
          if (!this.isLocal) {
            tableWithId = await createOrUpdateTable(tableName, tableData)
          } else {
            tableWithId = {...tableData, id: 'local'}
          }
          this.tables[tableName] = tableWithId
          return tableWithId
        }

      } else {
        return {...this.tables[tableName], error: `trying to send too fast, wait for ${waitDuration}ms`}
      }
    } else {
      const tableWithId = await createOrUpdateTable(tableName, tableData)
      this.tables[tableName] = tableWithId
      return tableWithId
    }
  }

}
