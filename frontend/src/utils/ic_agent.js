import { Actor, HttpAgent } from '@dfinity/agent'
import * as backend from './../../../../../backend/index.js'

export const createActor = async canisterId => {
  // test http://localhost:8000 main https://ic0.app
  const host = 'http://localhost:8000'
  const options = { agentOptions: { host } }
  const actor = backend.createActor(canisterId, options)
  return actor
}

export const read = async canisterId => {
  const actor = await createActor(canisterId)
  console.log(actor, 'actor')
  const data = await actor.read_files()
  console.log(data, 'read_files')
  return data;
}
