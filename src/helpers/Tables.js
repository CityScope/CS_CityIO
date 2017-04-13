import { deleteTable, getLatestTables, isTableRegistered, getLatestTable, createTable} from './api'
import { shouldWait } from './utils' 
import { emptyState } from '../config/constants'
import { Map, fromJS } from 'immutable'
import deepEqual from 'deep-equal'

export default class Tables{

  constructor(){
    // list of tabledata, key is tablename, each
    // containing the latest state of the table
    this.tables = new Map() 
    this.getAllTables()
  }

  getList () {
    // so we just need the keys
    return this.tables.keySeq().toArray()
  }

  getAllTables () {
    // usually just called once to
    // have it stored in memory
    return getLatestTables()
      .then(data=>{
        this.tables = fromJS(data) // converts to immutable object
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

  clearTable (tableName){
    return deleteTable(tableName)
      .then(()=>{
        if(this.tables.has(tableName)){
          this.tables = this.table.delete(tableName)
          console.log(`cleared table ${tableName}`)
        }
      })
      .then(()=>`cleared table ${tableName}`)
  }

  softClearTable(tableName){
    return deleteTable(tableName)
      .then(()=>{
        if(this.table.has(tableName)){
          createTable(tableName,this.tables.get(tableName).toJS())
            .then(newTableData=>{
                this.tables = this.tables.set(tableName, newTableData)
            })
        }
      })
  }

  updateTable (tableName,data) {

    if(!this.tables.has(tableName)){
      
      console.log(`created new table ${tableName}`)
      // create a new table 
      const tableDataWithId = createTable(tableName,data)
      this.tables = this.tables.set(tableName,fromJS(data))
    
    }else{
      
      console.log(this.tables.get(tableName))
      const prevTable = this.tables.get(tableName).toJS()

      // if its too soon to update
      if(shouldWait(prevTable.timestamp)) {
        // console.log(`** no push because not enough interval @ ${tableName}`)
        return
      }

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
}
