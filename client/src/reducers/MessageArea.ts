
import { ActionKind, Action } from "../actions";

export const MessageAreaReducer = (state = "", action: Action) => {
    switch (action.type) {
        case ActionKind.ChangedMessageArea: {
            if (action.payload !== undefined) {
                return action.payload.content || state;
            } else {
                return state;
            }
        }
        case ActionKind.ClearMessageArea: {
            return "";
        }
        default: {
            return state;
        }
    }
}
