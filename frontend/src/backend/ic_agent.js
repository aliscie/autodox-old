import { AuthClient } from "@dfinity/auth-client";
import { createActor, canisterId, idlFactory } from './../../../../../src/declarations/backend';

const { ic } = window;
const { plug } = ic;

let backendActor, loading = false

export async function identify() {
	const authClient = await AuthClient.create();
	if (await authClient.isAuthenticated()) {
		console.log('isAuthenticated')
		return authClient.getIdentity();
	}

	let identityProvider = "https://identity.ic0.app/#authorize";
	if (process.env.DFX_NETWORK != "ic") {
		identityProvider = `http://${process.env.IDENTITY_PROVIDER_ID}.localhost:8510/#authorize`
	}
	return await authClient.login({
		identityProvider,
		onSuccess: () => {
			window.location.reload()
		}
	});
}

export async function logout() {
	const authClient = await AuthClient.create();
	await authClient.logout()
}

export async function update_profile(username, image) {
	image = Array.from(image)
	console.log('image: ', image, typeof image)
	const actor = await get_actor()
	return await actor.update_profile({ username, image })
}

export async function get_profile() {
	const actor = await get_actor()
	let result = await actor.get_profile();
	console.log('get_profile#result: ', result)
	result = result[0];
	if (typeof (result.username) == "object") {
		result.username = result.username[0] || "";
	}
	if (typeof (result.image) == "object") {
		result.image = result.image[0] || "";
	}
	return result;
}

export async function is_logged() {
	const authClient = await AuthClient.create();
	return await authClient.isAuthenticated()
}

export const get_actor = async () => {
	await new Promise(resolve => !loading && resolve());
	loading = true

	if (!backendActor) {
		if (process.env.USE_WALLET) {
			let publicKey

			try {
				// const isConnected = await plug.isConnected();
				// if (!isConnected) {
				publicKey = await plug.requestConnect({
					whitelist: [process.env.BACKEND_CANISTER_ID],
					host: process.env.DFX_NETWORK === "ic" ? 'https://mainnet.dfinity.network' : 'http://localhost:8510',
					timeout: 50000,
					onConnectionUpdate: () => {
						console.log('sessionData: ', plug.sessionManager.sessionData)
					},
				});
				// }
			} catch (e) {
				console.log(e)
				return
			}

			backendActor = await plug.createActor({ canisterId, interfaceFactory: idlFactory, agent: plug.agent });
		} else {
			const authClient = await AuthClient.create();
			const identity = await authClient.getIdentity();
			backendActor = createActor(canisterId, {
				agentOptions: {
					identity,
					host: window.location.href,
				}
			});
		}
	}

	loading = false
	return backendActor;
}

export async function test_connect_wasm_bindgen() {
	let actor = await get_actor()
	return await actor.test_ic();
}

export async function create_directory() {
	let actor = await get_actor()
	return await actor.create_directory();
}

export async function get_directories() {
	let actor = await get_actor()
	return await actor.get_directories();
}


// export async function create_file() {
//     let actor = await get_actor()
//     return await actor.create_file();
// }

export async function register(username) {
	const backend = await get_actor()
	return await backend.register(username);
}

export async function get_current_user() {
	const actor = await get_actor()
	return await actor.get_current_user();
}

export async function get_test() {
	const actor = await get_actor()
	return await actor.get_test();
}
