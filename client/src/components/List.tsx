import * as React from "react";
import { Message } from "../types/Message";

const List = ({ list = [], handleClick }: any) => {
    return (
        <ul>
        {
            list.map((m: Message) => {
                return (<li
                    key={m.id}
                >
                        <span>{m.name}</span>
                        {" | "}
                        <span>{m.content}</span>
                </li>);
            })
        }
        </ul>
    );
};

export default List;
