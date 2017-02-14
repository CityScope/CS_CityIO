import { getLatestTables, isTableRegistered, getLatestTable, createTable} from './api' 
import { emptyState } from '../config/constants'
import { Map, fromJS } from 'immutable'

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
    // set the data to memory
    this.tables = this.tables.set(tableName,data)
    // const {head,tablePromise} = createTable(tableName,data)
    createTable(tableName,data)
    // return {tableName,id:head} // if we need the head
  }
}