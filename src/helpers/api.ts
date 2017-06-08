/// <reference path="../../typings/json.d.ts" />

// import { isLocal, ref } from '../config/constants'
import * as firebase from 'firebase-admin'
import * as serviceAccount from '../../firebase_credentials.json'
import { ref } from '../index'
import { emptyTable, ITable, ITables } from '../models/TableManager'

const databaseURL: string = 'https://cityio-db681.firebaseio.com'

export function connectToFirebase () {

  let ref: firebase.database.Reference

  console.log('** connecting to firebase')

  try {
    firebase.initializeApp({
      credential: firebase.credential.cert(serviceAccount),
      databaseURL,
    })
    ref = firebase.database().ref()
    console.log('** connected to firebase')
  } catch (e) {
    console.log('** failed connecting to firebase, switching to local mode')
    ref = null
  }
  return ref
}

export async function getTableNames (): Promise<string[]> {
  try {
    return await ref.child('heads').once('value')
    .then((snapshot) => Object.keys(snapshot.val()))
  } catch (e) {
    console.error(e.name, 'helper/api.ts:getTableNames')
    return Promise.resolve([])
  }
}

export async function isTableRegistered (tableName: string): Promise<boolean> {
  try {
    return await ref.child(`tables/${tableName}`).once('value')
      .then((screenshot) => screenshot.val() === null ? false : true)
  } catch (e) {
    console.error(e.name, 'helper/api.ts:isTableRegistered')
    return Promise.resolve(false)
  }
}

export async function getLatestTable (tableName): Promise<any> {
  let latestKey: string
  try {
    latestKey = await ref.child(`heads/${tableName}`).once
  ('value').then((snapshot) => snapshot.val())
  } catch (e) {
    console.error(e.name, 'helper/api.ts:getLatestTable, cannot get latest key')
    return Promise.resolve({...emptyTable, error: 'helper/api.ts:no latest key', timestamp: Date.now()})
  }

  try {
    return await ref.child(`tables/${tableName}/${latestKey}`).once('value').then((snapshot) => snapshot.val())
  } catch (e) {
    console.error(e.name, 'helper/api.ts:getLatestTable, cannot get latest table data')
    return Promise.resolve({...emptyTable, error: 'helper/api.ts:no latest table'})
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
  try {
    await ref.child(`tables/${tableName}`).set(null)
    await ref.child(`heads/${tableName}`).set(null)
  } catch (e) {
    console.error(e.name, 'helper/api.ts:dropTable')
  }
}

export async function createOrUpdateTable (tableName: string, tableData: ITable): Promise<ITable> {
  let newId: string
  try {
    newId = await ref.child(`tables/${tableName}`).push().key
  } catch (e) {
    console.error(e.name, 'helpers/api.ts:createOrUpdateTable 1')
    newId = 'localId'
  }

  const newTableData: ITable = { ...tableData, id: newId }

  try {
    await ref.child(`tables/${tableName}/${newId}`).set(newTableData)
    await ref.child(`heads/${tableName}`).set(newId)
  } catch (e) {
    console.error(e.name, 'helper/api.ts:createOrUpdateTable 2')
  }

  return Promise.resolve(newTableData)
}

export async function updateTimeStamp (tableName: string, tableId: string, timestamp: number): Promise<void> {
  try {
    await ref.child(`tables/${tableName}/${tableId}/timestamp`).set(timestamp)
  } catch (e) {
    console.error(e.name, 'helper/api.ts:updateTimeStamp')
  }
}
