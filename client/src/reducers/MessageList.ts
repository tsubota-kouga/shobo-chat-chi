import { ActionKind, Action } from "../actions";
import { Message, isMessageArray } from "../types/Message";

const initial: Array<Message> = [];

export function MessageListReducer(
    state: Array<Message> = initial,
    action: Action
): Array<Message> {
    switch (action.type) {
        case ActionKind.GetNewMessage: {
            const messages = action.payload.messages;
            if (isMessageArray(messages)) {
                return [...state, ...messages];
            }
        }
        case ActionKind.GetAllMessages: {
            const messages = action.payload.messages;
            if (isMessageArray(messages)) {
                return messages
            }

        }
        default: {
            return state;
        }
    }
}
