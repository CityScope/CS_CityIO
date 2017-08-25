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

TableManager.loadTables()
  .then(( createdTm ) => {
    tableManager = createdTm
})

app.use('/', ApiController)

app.use('/api', ApiController)

app.listen( PORT, () => {
  console.log(`listening to port  ${PORT}`)
})
