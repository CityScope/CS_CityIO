import { waitDuration } from '../config/constants'

export function shouldWait(timestamp){
  return Date.now() - timestamp < waitDuration
}