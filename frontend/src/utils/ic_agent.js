import {Actor, HttpAgent} from "@dfinity/agent";


export const createActor = async (canisterId) => {


    let agentOptions = {
        // host: "http://localhost:8000",
        // identity: await HttpAgent.createIdentity(),
    }

    const agent = new HttpAgent(agentOptions);


    let config = {
        agent,
        canisterId,
    }
    return "ok"
    //TODO
    // import * as deployerIDL from '../../../backend/backend.did.js'
    // return Actor.createActor(deployerIDL, config);
}
