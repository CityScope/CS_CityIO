## [welcome page](https://cityio.media.mit.edu) with links to available tables

https://cityio.media.mit.edu

## dev endpoints
endpoints in development will have /dev before the api

```https://cityio.media.mit.edu/dev/api/table/:tableName```

## list available tables
```
[GET] https://cityio.media.mit.edu/api/tables/list
```

## get table data
```
[GET] https://cityio.media.mit.edu/api/table/:tableName
```
params:

  tableName : name of table

format: json/application

https://cityio.media.mit.edu/table/:tableName is **deprecated**

## post table data
```
[POST] https://cityio.media.mit.edu/api/table/update/:tableName
```
params:

  tableName: name of table

  body

```text/plain``` or ```json/application``` format accepted.
  
  text will be converted to json internally, will throw an error if it's not
  valid json. Server welcomes any valid json, but only things inside ```objects``` and ```grid``` objects
  will be used for comparison to detect uniqueness.

https://cityio.media.mit.edu/table/update/:tableName is **deprecated** 

## delete table data
```
[GET] https://cityio.media.mit.edu/api/table/clear/:tablename
```
params:
  
  tableName: name of table

**be careful! will delete all data from memory cache and DB** 

## delete module data
```
[GET] https://cityio.media.mit.edu/api/table/clear/:tablename/:modulename
```
params:
  
  tablename: name of table
  modulename: name of module

**be careful! will delete all data from memory cache and DB** 

