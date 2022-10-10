import {AuthClient} from "@dfinity/auth-client";

export async function identify() {
    const authClient = await AuthClient.create();
    if (authClient.isAuthenticated()) {
        return authClient.getIdentity();
    } else {
        authClient.login({
            identityProvider: 'https://identity.ic0.app/#authorize',
            onSuccess: () => {
                //TODO this does not work
                // it return empty object like this "{}"
                return authClient.getIdentity();
            }
        })
    }
}