import * as React from "react";
import MessageList from "./containers/MessageList";
import NameInput from "./containers/NameInput";
import MessageArea from "./containers/MessageArea";
import SendButton from "./containers/SendButton";
import UpdateButton from "./containers/UpdateButton";

const App: React.FC = () => {
  return (
    <div>
        <MessageList />
        <NameInput />
        <MessageArea />
        <SendButton />
        <UpdateButton />
    </div>
  );
};

export default App;
