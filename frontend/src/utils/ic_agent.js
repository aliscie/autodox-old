import { Actor, HttpAgent } from '@dfinity/agent'
import * as backend from '../../../src/declarations/backend/index'

export const createActor = async canisterId => {
  // test http://localhost:8000 main https://ic0.app
  const host = 'http://localhost:8000'
  const identity = await HttpAgent.createIdentity()
  const options = identity
    ? { agentOptions: { host, identity } }
    : { agentOptions: { host } }

  return backend.createActor(canisterId, options)
}
