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
      console.log(value)
      return value
    })
}

function updateHead (tableName,newHead) {
  return ref.child(`heads/${tableName}`).set(newHead)
}

export function createTable (tableName,tableData) {
  const head = ref.child(`tables/${tableName}`).push().key
  return ref.child(`tables/${tableName}/${head}`).set(tableData)
    .then(()=>updateHead(tableName,head))
    .then(()=>({tableName,head}))
}

export function updateTable (tableName,tableData) {
  return createTable(tableName, tableData)
}

export function getTableList () {
  return ref.child('heads').once('value')
    .then(screenshot=>Object.keys(screenshot.val()))
}