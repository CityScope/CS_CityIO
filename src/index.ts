import { json, raw, urlencoded  } from 'body-parser'
import * as express from 'express'
import { resolve } from 'path'

import { PORT } from './config/constants'
import { ApiController } from './controllers'
import TableManager from './models/TableManager'

const app: express.Application = express()

app.use(json())
app.use(raw({type: 'text/plain'}))

export let tableManager: TableManager
export let ref: any

TableManager.loadTables(process.argv[3] === 'local')
  .then(( createdTm ) => {
    tableManager = createdTm
    ref = tableManager.getRef()
})

app.use('/', ApiController)

app.use('/api', ApiController)

app.listen( PORT, () => {
  console.log(`listening to port  ${PORT}`)
})
