import { ref } from '../config/constants'

import { emptyTable, ITable, ITables } from '../models/TableManager'

export async function getTableNames (): Promise<string[]> {
  try {
    return await ref.child('heads').once('value')
    .then((snapshot) => Object.keys(snapshot.val()))
  } catch (e) {
    console.error(e)
    return Promise.resolve([])
  }
}

export async function isTableRegistered (tableName: string): Promise<boolean> {
  try {
    return await ref.child(`tables/${tableName}`).once('value')
      .then((screenshot) => screenshot.val() === null ? false : true)
  } catch (e) {
    console.error(`error checking existance of ${tableName}`)
    console.log(e)
    return Promise.resolve(false)
  }
}

export async function getLatestTable (tableName): Promise<any> {
  let latestKey: string
  try {
    latestKey = await ref.child(`heads/${tableName}`).once
  ('value').then((snapshot) => snapshot.val())
  } catch (e) {
    console.error(`error fetching latest key for ${tableName}`)
    console.error(e)
    return Promise.resolve(emptyTable)
  }

  try {
    return await ref.child(`tables/${tableName}/${latestKey}`).once('value').then((snapshot) => snapshot.val())
  } catch (e) {
    console.error(`error fetching latest data for ${tableName}/${latestKey}`)
    console.error(e)
  }
}

export async function getLatestTables (): Promise<ITables> {
  const tableNames: string[] = await getTableNames()
  const tables: ITables = {}

  for (const tableName of tableNames) {
    tables[tableName] = await getLatestTable(tableName)
  }

  return tables
}

export async function dropTable (tableName: string): Promise<void> {
  await ref.child(`tables/${tableName}`).set(null)
  await ref.child(`heads/${tableName}`).set(null)
}

export async function createOrUpdateTable (tableName: string, tableData: ITable): Promise<ITable> {
  const newId: string = await ref.child(`tables/${tableName}`).push().key

  const newTableData: ITable = { ...tableData, id: newId }

  try {
  await ref.child(`tables/${tableName}/${newId}`).set(newTableData)
  await ref.child(`heads/${tableName}`).set(newId)
  } catch (e) {
    console.error('error adding/updating table data')
    console.error(e)
    return Promise.resolve(null)
  }

  return Promise.resolve(newTableData)
}

export async function updateTimeStamp (tableName: string, tableId: string, timestamp: number): Promise<void> {
  await ref.child(`tables/${tableName}/${tableId}/timestamp`).set(timestamp)
}
