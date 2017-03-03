import { ref } from '../config/constants'

export function isTableRegistered (tableName) {
  return ref.child(`tables/${tableName}`).once('value')
    .then(screenshot=>screenshot.val() === null ? false : true)
}

function getTableHead (tableName) {
  return ref.child(`heads/${tableName}`).once('value')
    .then(screenshot=>screenshot.val())
}

export function getLatestTable (tableName) {
  return getTableHead(tableName)
    .then(head=>ref.child(`tables/${tableName}/${head}`).once('value'))
    .then(screenshot => screenshot.val())
    .then(value=>{
      // console.log(value)
      return value
    })
}

export function getLatestTables () {
  let tableDataAll = {}
  let tableNameList = []
  return getTableList()
    .then(tableNames=>{
      tableNameList = tableNames
      const promises = tableNameList.map(
        tableName=>getLatestTable(tableName)
        )
      return Promise.all(promises)
    })
    .then(tableData=>{
      tableNameList.map((tableName,index)=>{
        tableDataAll[tableName]=tableData[index]
      })
      return tableDataAll
    })
}

export function deleteTable (tableName) {
  const promises = [
    ref.child(`heads/${tableName}`).set(null),
    ref.child(`tables/${tableName}`).set(null)
  ]
  return Promise.all(promises)
}

function updateHead (tableName,newHead) {
  return ref.child(`heads/${tableName}`).set(newHead)
}

export function createTable (tableName,tableData) {
  const head = ref.child(`tables/${tableName}`).push().key
  const tableDataWithId = {...tableData, id:head, timestamp:Date.now()}
  ref.child(`tables/${tableName}/${head}`).set(tableDataWithId)
    .then(()=>updateHead(tableName,head))
    .catch(error=>{console.log(error)})
  return tableDataWithId
}

export function updateTable (tableName,tableData) {
  return createTable(tableName, tableData)
}

export function getTableList () {
  return ref.child('heads').once('value')
    .then(screenshot=>Object.keys(screenshot.val()))
}