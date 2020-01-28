
import {
    MessageBeforeSend,
    Message,
    isMessageArray } from "../types/Message";

const apiEndPoint = `http://${window.location.host}/messages`

export async function getAllMessages(): Promise<Array<Message>> {
    const res = await fetch(apiEndPoint, {
        method: "GET",
    });
    const data = await res.json();
    if (isMessageArray(data)) {
        return data;
    }
    throw new Error("Response data is not Messages");
}

export async function postMessage(message: MessageBeforeSend): Promise<boolean> {
    const res = await fetch(apiEndPoint, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(message),
    });
    return res.status === 200;
}

