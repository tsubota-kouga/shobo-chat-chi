import { connect } from "react-redux";
import List from "../components/List";

function mapStateToProps(state: any) {
    return {
        list: state.MessageListReducer
    };
}

export default connect(
    mapStateToProps,
)(List);
