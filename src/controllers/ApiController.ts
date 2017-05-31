import axios, { AxiosError, AxiosResponse } from 'axios'
import { Request, Response, Router } from 'express'
import { getTableNames } from '../helpers/api'
import TableManager, { emptyTable, ITable } from '../models/TableManager'

const router: Router = Router()

let tableManager: TableManager

TableManager.loadTables()
  .then((tm) => { tableManager = tm })

/*
 * get table name
 * */
router.get('/', async (req: Request, res: Response) => {
  const tableNames = await getTableNames()

  const links = tableNames.reduce((result, tn) => {
    return result + `<p><a href="https://cityio.media.mit.edu/table/${tn}">${tn}</a></p>`
  }, '')

  res.send(`
    <h1>tables</h1>
    <p><img src="http://weknowyourdreams.com/images/unicorn/unicorn-08.jpg"></p>
    ${links}
    `)
})

/*
 * get table data
 * */
router.get('/table/:tableName', async (req: Request, res: Response) => {
  const tableName: string = req.params.tableName
  const table: ITable = await tableManager.getTable(tableName)
  res.json(table)
})

/*
 * post table data
 * TODO: get rid of update
 * */
router.post('/table/update/:tableName', async (req: Request, res: Response) => {
  const tableName: string = req.params.tableName
  let tableData: any

  switch (req.headers['content-type']) {
    case 'application/json' :
      tableData = req.body
      break
    case 'text/plain' :
    default :
      try {
        tableData = JSON.parse(req.body.toString('utf8'))
      } catch (e) {
        if (e instanceof SyntaxError) {
          res.status(500)
            .send(`
            <h1> Invalid JSON </h1>
            <p> check data </p>
            <p> ${e.name}: ${e.message} </p>
              `)
          return
        }
      }
      break
  }
  const formattedTableData: ITable = {...emptyTable, ...tableData, timestamp: Date.now()}
  // console.log(formattedTableData)
  const newTable: ITable = await tableManager.addTable(tableName, formattedTableData)

  res.json(newTable)
})

export const ApiController: Router = router
