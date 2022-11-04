import {Actor, HttpAgent} from "@dfinity/agent";

// a function that create ic agent actor
export const createActor = async (canisterId, agentOptions) => {
    const agent = new HttpAgent(agentOptions);
    const actor = Actor.createActor(canisterId, {
        agent,
        canisterId,
    });
    return actor;
}
