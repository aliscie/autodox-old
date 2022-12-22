import {AuthClient} from "@dfinity/auth-client";
import * as backend from './../../../../../backend/index.js'

// export const createActor = async canisterId => {
//     // test http://localhost:8000 main https://ic0.app
//     const host = 'http://localhost:8000'
//     const options = {agentOptions: {host}}
//     return backend.createActor(canisterId, options)
// }

// export const read = async canisterId => {
//     const actor = await createActor(canisterId)
//     return await actor;
// }

export const create_file = async (canisterId, text) => {
    const actor = await createActor(canisterId)
    return await actor.create_file(text);
}

export const create_directory = async (canisterId, text) => {
    const actor = await createActor(canisterId)
    return await actor.create_directory(id, root, vertices, adjacency, name);
}

export async function identify() {
    const authClient = await AuthClient.create();
    if (await authClient.isAuthenticated()) {
        return authClient.getIdentity();
    }
    return await authClient.login({identityProvider: "https://identity.ic0.app"});
}


export async function logout() {
    const authClient = await AuthClient.create();
    await authClient.logout()
}


export async function is_logged() {
    const authClient = await AuthClient.create();
    return await authClient.isAuthenticated()
}