
import { AuthClient } from "@dfinity/auth-client";

export async function identify() {
    const authClient = await AuthClient.create();
    console.log(authClient);
    if (authClient.isAuthenticated()) {
        return "already logged in";
    } else {
        await authClient.login({
            identityProvider:
        process.env.DFX_NETWORK === "ic"
          ? "https://identity.ic0.app/#authorize"
          : `http://${process.env.INTERNET_IDENTITY_CANISTER_ID}.localhost:8000/#authorize`,
      onSuccess:
        async () => {
                return "login success";
            },
        });
    }
}