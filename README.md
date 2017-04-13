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

1. install [yarn](https://yarnpkg.com/en/docs/install)

  yarn is a package manager for node applications
2. clone and cd to this repo
3. get firebase admin credential file

  there are two ways doing this
  1. (easy) ask Yasushi

    or
  2. Create a new project in [firebase](https://firebase.google.com/)
    1. create project
    2. change inside ```src/config/constants.js```

      ```javascript
      const databaseURL = 'https://cityio-db681.firebaseio.com'
      ```
      to your firebase url
    3. in the firebase console, navigate to 
      
      ```
      Settings (gear button next to ) > "SERVICE ACCOUNTS" tab
      download file by clicking "GENERATE NEW PRIVATE KEY"
      ```
    4. rename the key file to 'firebase_credentials.json'

4. put the file in ```src/config``` directory

4. run ```yarn```

5. run ```yarn start```

6. check ```localhost:8080```


