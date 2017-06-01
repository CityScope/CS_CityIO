cityio server program

# API

```
[GET] https://cityio.media.mit.edu
```
[welcome page](https://cityio.media.mit.edu) having links to available tables

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
[POST] https://cityio.media.mit.edu/api/update/:tableName
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
[GET] https://cityio.media.mit.edu/api/clear/:tablename
```
params:
  
  tableName: name of table

**becareful! will delete all data from memory cache and DB** 


