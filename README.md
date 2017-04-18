# CityIO Server

the server script for cityscope platforms to send their data and make accessible
to other clients (using, GAMA, Unity, Processing, Rhinoceros/Grasshopper, or anything using HTTP).

KISS

## API documentation

the current main server is [http**s**://cityio.media.mit.edu](https://cityio.media.mit.edu/).
if running locally, substitute the hostname to localhost:PORT (ex. http://localhost:8080/)


### get list of tables
  ```
  (GET) https://cityio.media.mit.edu/
  ```

### get latest table data
  ```
  (GET)  https://cityio.media.mit.edu/table/:tableName
  ```

### post table data
  ```
  (POST)  https://cityio.media.mit.edu/table/upate/:tableName
  ```
  body should either be raw or json

### [CAUTION] reset database
  ```
  (GET)  https://cityio.media.mit.edu/tables/hardReset
  ```
  drop everything in firebase.

## how to run it locally

1. install [node & npm](https://nodejs.org/en/download/package-manager/)

2. clone this repo

3. get firebase admin credential file
  
    there are two ways doing this depending on your database preferences. If you wish to have use the same database
  
      1. (easy) ask Yasushi,
    
    or if you wish to have your own database

      1. Create a new project in [firebase](https://firebase.google.com/)
          1. create project
          2. change inside ```src/config/constants.js```

    ```javascript
    const databaseURL = 'https://cityio-db681.firebaseio.com'
    ```
    to your firebase url
      
      2. in the firebase console, navigate to Settings (gear button next to ) > "SERVICE ACCOUNTS" tab download file by clicking "GENERATE NEW PRIVATE KEY"

      3. rename the key file to 'firebase_credentials.json'

4. put the file in ```src/config``` directory

5. open the terminal of your choice, ```cd``` to cloned repository

6. run ```npm install```

    let it install the dependencies
7. run ```npm run start```

    the server script will start
8. check ```localhost:8080```


