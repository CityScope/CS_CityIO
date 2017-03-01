import { getLatestTables, isTableRegistered, getLatestTable, createTable} from './api' 
import { emptyState } from '../config/constants'
import { Map, fromJS } from 'immutable'
import deepEqual from 'deep-equal'

export default class Tables{

  constructor(){
    // list of table names
    this.tables = new Map()
    this.getAllTables()
  }

  getList () {
    return this.tables.keySeq().toArray()
  }

  getAllTables () {
    return getLatestTables()
      .then(data=>{
        this.tables = fromJS(data)
        console.log('** ready **')
      })
  }

  getTable (tableName) {
    if(this.tables.has(tableName)){
      // it was in memory
      return Promise.resolve(this.tables.get(tableName))
    }else{
      // it was not in memory
      return isTableRegistered(tableName)
        .then(result=>
          result
          ? getLatestTable(tableName)
          : {...emptyState,error:`cannot find ${tableName}`})
      }
  }

  updateTable (tableName,data) {

    // check if its the same
    const prevTable = this.tables.get(tableName).toJS()
    delete prevTable.id
    delete prevTable.timestamp

    if(!deepEqual(prevTable,data)){
      const tableDataWithId = createTable(tableName,data)
      this.tables = this.tables.set(tableName,fromJS(tableDataWithId))
      // console.log(`** updated table ${tableName}**`)
    }else{
      // console.log(`** no push because same data @ ${tableName}**`)
    }
  }

}