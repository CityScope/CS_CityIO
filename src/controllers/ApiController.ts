import axios, { AxiosError, AxiosResponse } from 'axios'
import { Request, Response, Router } from 'express'
import { baseURL } from '../config/constants'
import { html } from '../helpers/html'
import { tableManager } from '../index'
import TableManager, { emptyTable, ITable } from '../models/TableManager'
const router: Router = Router()

/*
 * get table name
 * */
router.get('/', async (req: Request, res: Response) => {
  const tableNames = tableManager.getList()

  const links = tableNames.reduce((result, tn) => {
    return result + `<li><a href="${baseURL}/api/table/${tn}">${tn}</a></li>`
  }, '')

  res.send(html(`
    <img src="http://www.vgmuseum.com/end/nes/a/venuswar-3.png" width="512">
    <p>available tables and json</p>
    <ul>
    ${links}
    </ul>
    <p> more on api and documenation for the server:
      <a href="https://github.com/mitmedialab/cityioserver">github repository</a>
    </p>
    `))
})

/*
 * get table data
 * */
router.get('/table/:tableName', async (req: Request, res: Response) => {
  const tableName: string = req.params.tableName
  const table: ITable = await tableManager.getTable(tableName)
  res.jsonp(table)
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
  // the table data does not contain the id yet.
  const formattedTableData: ITable = {...emptyTable, ...tableData, timestamp: Date.now()}
  // console.log(formattedTableData)
  const newTable: ITable = await tableManager.addTable(tableName, formattedTableData)

  res.jsonp(newTable)
})

/*
 * clear table data
 * */
router.get('/table/clear/:tableName', async (req: Request, res: Response) => {
  const tableName: string = req.params.tableName
  await tableManager.clearTable(tableName)
  res.jsonp(`cleared ${tableName}`)
})

export const ApiController: Router = router
