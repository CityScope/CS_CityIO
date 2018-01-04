import { json, raw, urlencoded  } from 'body-parser'
import * as express from 'express'
import { join, resolve } from 'path'

import { frontendDir, PORT } from './config/constants'
import { ApiController } from './controllers'
import TableManager from './models/TableManager'

const app: express.Application = express()
const rootDir: string = resolve(__dirname, '../')

app.use(json())
app.use(raw({type: 'text/plain'}))

export let tableManager: TableManager

TableManager.loadTables()
  .then(( createdTm ) => {
    tableManager = createdTm
})

// api endpoints
app.use('/api', ApiController)

app.use(express.static(frontendDir))

app.listen( PORT, () => {
console.log(`listening to port  ${PORT}`)
})
