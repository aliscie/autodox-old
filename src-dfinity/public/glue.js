import {Actor, HttpAgent} from "@dfinity/agent";

import {idlFactory} from './declarations/autodox/autodox.did.js';

export {idlFactory} from './declarations/autodox/autodox.did.js';
export const canisterId = process.env.AUTODOX_CANISTER_ID;

export const createActor = (canisterId, options) => {
    const agent = new HttpAgent({...options?.agentOptions});

    if (process.env.NODE_ENV !== "production") {
        agent.fetchRootKey().catch(err => {
            console.warn("Unable to fetch root key. Check to ensure that your local replica is running");
            console.error(err);
        });
    }

    return Actor.createActor(idlFactory, {
        agent,
        canisterId,
        ...options?.actorOptions,
    });
};

export const autodox = createActor(canisterId);

export function say_hello() {
    return "Hello dfinity ic 2"
}