/// <reference path="../../typings/json.d.ts" />

import * as firebase from 'firebase-admin'
import * as serviceAccount from '../../firebase_credentials.json'

//
// general
//

export const isDebug: boolean = process.argv[2] === 'debug'
export const baseURL: string = isDebug ? 'http://localhost:8080' : 'https://cityio.media.mit.edu'
export const PORT: number = 8080
export const waitDuration: number = 500 // ms

let didConnect: boolean = process.argv[3] !== 'local'

//
// firebase
//

const databaseURL: string = 'https://cityio-db681.firebaseio.com'

try {
  firebase.initializeApp({
    credential: firebase.credential.cert(serviceAccount),
    databaseURL,
  })} catch (e) {
 didConnect = false
}

export const emptyState = {
  error: '',
  grid: [],
  objects: {},
}

if (!didConnect) {
  console.log('GOING LOCAL')
}

export const ref: any = didConnect ? firebase.database().ref() : {}
export const isLocal = !didConnect
