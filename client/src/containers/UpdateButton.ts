import { connect } from "react-redux";

import { getAllMessagesAction } from "../actions";
import Button from "../components/Button";

function mapStateToProps() {
    return {
        payload: {},
        value: "Update",
    };
}

function mapDispatchToProps(dispatch: any) {
    return {
        handleClick: (payload: any) => {
            dispatch(getAllMessagesAction());
        },
    }
}

export default connect(
    mapStateToProps,
    mapDispatchToProps,
)(Button);
