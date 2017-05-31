/// <reference path="../../typings/json.d.ts" />

import * as firebase from 'firebase-admin'
import * as serviceAccount from '../../firebase_credentials.json'

//
// general
//

export const PORT: number = 8080
export const waitDuration: number = 500 // ms

//
// firebase
//

const databaseURL: string = 'https://cityio-db681.firebaseio.com'

firebase.initializeApp({
  credential: firebase.credential.cert(serviceAccount),
  databaseURL,
})

export const emptyState = {
  error: '',
  grid: [],
  objects: {},
}

export const ref: firebase.database.Reference = firebase.database().ref()
