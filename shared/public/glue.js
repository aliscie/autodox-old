//shared
export const invoke = window.__TAURI__.invoke
export const emit = window.__TAURI__.emit;

export async function invokeHello(name) {
    return await invoke("hello", {name: name});
}

export function invokerFunction(command, args){
    if(!args){
        return invoke(command);
    }
    // if we got some args parse them into json objects!
    return invoke(command, JSON.parse(args));
}

export async function invokerAsyncFunction(command, args){
    if(!args){
        return invoke(command);
    }
    return invoke(command, JSON.parse(args));
}
