
import { ActionKind, Action } from "../actions";

export function NameInputReducer(state = "", action: Action): string {
    switch (action.type) {
        case ActionKind.ChangedNameInput: {
            if (action.payload !== undefined) {
                return action.payload.name || state;
            } else {
                return state;
            }
        }
        case ActionKind.ClearNameInput: {
            return "";
        }
        default: {
            return state;
        }
    }
}
