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
  let tableData;
  switch (req.headers['content-type']){
    case 'application/json':
      tableData = req.body
      break
    case 'text/plain':
    default :
      try{
        tableData = JSON.parse(req.body.toString('utf8'))
      }catch(e){
        if(e instanceof SyntaxError){
          // res.json(['Invalid JSON, check the table data',e.name,e.message])
          res.status(500).send(`
            <h1>Invalid JSON data</h1>
            <p>server could not parse the table data</p>
            <p>check the json data you are sending</p>
            <p>${e.name}: ${e.message}</p>`)
          return
        }
      }
    break
  }

  tables.updateTable(tableName,tableData)
  res.json([`updated ${tableName}`])

})


app.listen(PORT,()=>{
  console.log(`server started @ port ${PORT}`)
})