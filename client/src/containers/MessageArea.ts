import { connect } from "react-redux";

import { changedMessageAreaAction } from "../actions";
import TextArea from "../components/TextArea";

function mapStateToProps(state: any) {
    return {
        value: state.MessageAreaReducer
    };
}

function mapDispatchToProps(dispatch: any) {
    return {
        handleChange: (value: string) => {
            return dispatch(changedMessageAreaAction(value));
        }
    }
}

export default connect(
    mapStateToProps,
    mapDispatchToProps
)(TextArea);
