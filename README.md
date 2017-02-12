# CityIO Server
Node.js version
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

## API documentation

### get list of tables
  ```
  (GET) localhost:8080/
  ```

### get latest table data
  ```
  (GET) localhost:8080/table/:tableName
  ```

### post table data
  ```
  (POST) localhost:8080/table/upate/:tableName
  ```
  body should either be raw or json