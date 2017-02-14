import express from 'express'
import { firebase, PORT } from './config/constants'
import bodyParser from 'body-parser'
import { 
  isTableRegistered,
  getLatestTable,
  updateTable,
  createTable,
  getTableList,
   } from './helpers/api'
import { emptyState } from './config/constants'
import Tables from './helpers/Tables'

let tables = new Tables()


const app = express()
app.use(bodyParser.json())
app.use(bodyParser.raw({type:'text/plain'}))

app.get('/',(req,res)=>{
  // get current registered tables
  res.json(tables.getList())
})

app.get('/table/:tableName',(req,res)=>{
  // get the latest state for a given table
  const tableName = req.params.tableName
  tables.getTable(tableName)
    .then((tableData)=>res.json(tableData))
})

app.post('/table/update/:tableName/',(req,res)=>{
  const tableName = req.params.tableName
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
  tables.updateTable(tableName,tableData)
  res.json([`updated ${tableName}`])
})


app.listen(PORT,()=>{
  console.log(`server started @ port ${PORT}`)
})