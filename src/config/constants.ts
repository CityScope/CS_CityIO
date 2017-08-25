
//
// general
//

export const isDebug: boolean = process.argv[2] === 'debug'
export const baseURL: string = isDebug ? 'http://localhost:8080' : 'https://cityio.media.mit.edu'
export const PORT: number = 8080
export const waitDuration: number = 500 // ms

// let diddeConnect: boolean = process.argv[3] !== 'local'

//
// firebase
//

export const emptyState = {
  error: '',
  grid: [],
  objects: {},
}
