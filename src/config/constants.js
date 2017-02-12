import path from 'path'

export const firebase = require("firebase-admin");

const databaseURL = 'https://cityio-db681.firebaseio.com'
const credentialFilePath =path.resolve(__dirname,'firebase_credentials.json')

firebase.initializeApp({
  credential: firebase.credential.cert(credentialFilePath),
  databaseURL
});

export const emptyState = {
  grid:[],
  objects:{},
  error:'',
}

export const ref = firebase.database().ref()