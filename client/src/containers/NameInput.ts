import { connect } from "react-redux";
import { changedNameInputAction } from "../actions";
import Input from "../components/Input";

function mapStateToProps(state: any) {
    return {
        value: state.NameInputReducer,
        type: "text",
    };
}

function mapDispatchToProps(dispatch: any) {
    return {
        handleChange: (name: string) => {
            dispatch(changedNameInputAction(name));
        }
    }
}

export default connect(
    mapStateToProps,
    mapDispatchToProps
)(Input);

