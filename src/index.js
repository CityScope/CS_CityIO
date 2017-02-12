import express from 'express'
import firebase from './config/constants'
import bodyParser from 'body-parser'
import { 
  isTableRegistered,
  getLatestTable,
  updateTable,
  createTable,
  getTableList,
   } from './helpers/api'
import {
  emptyState
} from './config/constants'

const app = express()
//app.use(bodyParser.raw())
app.use(bodyParser.json())
app.use(bodyParser.raw({type:'text/plain'}))

const port = 8080

app.get('/',(req,res)=>{
  // get current registered tables
  getTableList()
    .then(tableList=>{res.json(tableList)}) 
})

app.get('/table/:tableName',(req,res)=>{
  // get the latest state for a given table
  const tableName = req.params.tableName

  isTableRegistered(tableName)
    .then(result=>
      result
      ? getLatestTable(tableName).then(tableData=>res.json(tableData))
      : res.json({...emptyState,error:`cannot find ${tableName}`})
      )
})

app.post('/table/update/:tableName/',(req,res)=>{

  let tableData

  switch (req.headers['content-type']){
    case 'application/json':
      tableData = req.body
      break
    case 'text/plain':
    default :
      tableData = JSON.parse(req.body.toString('utf8'))
    break
  }

  const tableName = req.params.tableName
  isTableRegistered(tableName)
    .then(isRegistered=>
      isRegistered
      ? updateTable(tableName,tableData)
      : createTable(tableName,tableData)
      )
    .then(response=>{res.json(response)})
})


app.listen(port,()=>{
  console.log(`server started @ port ${port}`)
})