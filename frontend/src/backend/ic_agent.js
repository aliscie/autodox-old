import {AuthClient} from "@dfinity/auth-client";
import {createActor, backend} from './../../../../../src/declarations/backend';

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

export async function update_profile(data) {
    const actor = await createActor(canisterId)
    return await actor.update_profile(data);
}

export async function get_profile() {
    const actor = await createActor(canisterId)
    return await actor.get_profile();
}

export async function is_logged() {
    const authClient = await AuthClient.create();
    return await authClient.isAuthenticated()
}


export const get_actor = async canisterId => {
    // test http://localhost:8000 main https://ic0.app
    const host = "https://ic0.app"
    const options = {agentOptions: {host}}
    return createActor(canisterId, options)
}


export async function test_connect_wasm_bindgen() {
    let actor = await get_actor(process.env.BACKEND_CANISTER_ID)
    // console.log("actor", actor)
    // console.log("backend",backend)
    return await backend.test_ic();
    return await actor.test_ic();
}

export async function register(username) {
    return await backend.register(username);
}

