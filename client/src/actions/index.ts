import { MessageBeforeSend, Message } from "../types/Message";
import {
    getAllMessages,
    postMessage } from "../http";

export enum ActionKind {
// MessageList
    GetNewMessage,
    GetAllMessages,
// Input
    ChangedNameInput,
    ClearNameInput,
// TextArea
    ChangedMessageArea,
    ClearMessageArea,
    SendMessage,
}

export interface Action {
    type: ActionKind;
    payload?: any;
}

// MessageList
export function GetNewMessageAction(messages: Array<Message>) {
    return {
        type: ActionKind.GetNewMessage,
        payload: {
            messages: messages
        }
    }
}

export function getAllMessagesAction() {
    return async (dispatch: any) => {
        try {
            const messages = await getAllMessages();
            dispatch({
                type: ActionKind.GetAllMessages,
                payload: {
                    messages
                }
            });
        } catch (err) {
            console.log(err);
        }
    }
}

// Input
export function changedNameInputAction(name: string) {
    return {
        type: ActionKind.ChangedNameInput,
        payload: {
            name,
        }
    };
}
export function clearNameInputAction(): Action {
    return {
        type: ActionKind.ClearNameInput,
        payload: {}
    }
}

// TextArea
export function changedMessageAreaAction(content: string): Action {
    return {
        type: ActionKind.ChangedMessageArea,
        payload: {
            content
        }
    };
}

export function clearMessageAreaAction(): Action {
    return {
        type: ActionKind.ClearMessageArea,
    };
}

export function sendMessageAction(message: MessageBeforeSend) {
    return async (dispatch: any) => {
        try {
            const status = await postMessage(message);
            dispatch({
                type: ActionKind.SendMessage,
                payload: {
                    status
                }
            });
        } catch (err) {
            console.log(err);
        }
    }
}

