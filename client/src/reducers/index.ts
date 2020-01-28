import { combineReducers } from "redux";

import { MessageListReducer } from "./MessageList";
import { NameInputReducer } from "./NameInput";
import { MessageAreaReducer } from "./MessageArea";

export default combineReducers({
    MessageListReducer,
    NameInputReducer,
    MessageAreaReducer,
});
