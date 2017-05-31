import { json, raw, urlencoded  } from 'body-parser'
import * as express from 'express'
import { resolve } from 'path'

import { PORT } from './config/constants'
import { ApiController } from './controllers'

const app: express.Application = express()

app.use(json())
app.use(raw({type: 'text/plain'}))

app.use('/', ApiController)

app.listen( PORT, () => {
  console.log(`listening to port  ${PORT}`)
})
