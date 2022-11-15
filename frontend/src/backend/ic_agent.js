import * as backend from './../../../../../backend/index.js'

const createActor = async canisterId => {
    // test http://localhost:8000 main https://ic0.app
    const host = 'http://localhost:8000'
    const options = {agentOptions: {host}}
    const actor = backend.createActor(canisterId, options)
    return actor
}

export const call_actor = async (canisterId, method, args = null) => {
    const actor = await createActor(canisterId)
    if (args == null) {
        return eval(`actor.${method}()`)
    }
    return eval(`actor.${method}(${args})`)

}
