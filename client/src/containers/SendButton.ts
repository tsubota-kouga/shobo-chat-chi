import { connect } from "react-redux";

import { MessageBeforeSend, isMessageBeforeSend, isCompleted } from "../types/Message";
import {
    sendMessageAction,
    clearMessageAreaAction,
    clearNameInputAction } from "../actions";
import Button from "../components/Button";

function mapStateToProps(state: any) {
    return {
        payload: {
            name: state.NameInputReducer,
            content: state.MessageAreaReducer,
        },
        value: "Send",
    };
}

function mapDispatchToProps(dispatch: any) {
    return {
        handleClick: (message: unknown) => {
            if (isMessageBeforeSend(message) &&
                isCompleted(message)) {
                dispatch(sendMessageAction(message));
                dispatch(clearMessageAreaAction());
                dispatch(clearNameInputAction());
            }
        }
    }
}

export default connect(
    mapStateToProps,
    mapDispatchToProps
)(Button);
