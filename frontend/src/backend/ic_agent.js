import { AuthClient } from "@dfinity/auth-client";
import { createActor, canisterId, idlFactory } from './../../../../../src/declarations/backend';
import { Actor, HttpAgent } from "@dfinity/agent";

const { ic } = window;
const { plug } = ic;

let backendActor

export async function identify() {
	const authClient = await AuthClient.create();
	if (await authClient.isAuthenticated()) {
		return authClient.getIdentity();
	}

	let identityProvider = "https://identity.ic0.app/#authorize";
	if (process.env.DFX_NETWORK != "ic") {
		identityProvider = "http://r7inp-6aaaa-aaaaa-aaabq-cai.localhost:8510/#authorize"
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
	return await actor.get_profile();
}

export async function is_logged() {
	const authClient = await AuthClient.create();
	return await authClient.isAuthenticated()
}

export const get_actor = async () => {
	if (!backendActor) {
		console.log('USE_WALLET: ', process.env.USE_WALLET)

		if (process.env.USE_WALLET) {
			let publicKey

			try {
				publicKey = await plug.requestConnect({
					whitelist: [process.env.BACKEND_CANISTER_ID],
					host: process.env.DFX_NETWORK === "ic" ? 'https://mainnet.dfinity.network' : 'http://localhost:8510',
					timeout: 50000,
					onConnectionUpdate: () => {
						console.log('sessionData: ', plug.sessionManager.sessionData)
					},
				});
			} catch (e) {
				console.error(e)
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

	return backendActor;
}

export async function test_connect_wasm_bindgen() {
	let actor = await get_actor()
	return await actor.test_ic();
}

export async function register(username) {
	const backend = await get_actor()
	return await backend.register(username);
}
