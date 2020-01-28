
export interface MessageBeforeSend {
    name: string;
    content: string;
}

export function isMessageBeforeSend(message: any): message is MessageBeforeSend {
    return typeof message.name === "string" &&
           typeof message.content === "string";
}

export function isCompleted(message: MessageBeforeSend): boolean {
    return (message.name.trimLeft() !== "" &&
            message.content.trimLeft() !== "");
}

export type Message = MessageBeforeSend & {
    id: string;
    time: number;
};

export function isMessage(message: any): message is Message {
    return typeof message.id === "string" &&
           typeof message.time === "number" &&
           isMessageBeforeSend(message);
}

export function isMessageArray(messages: any): messages is Array<Message> {
    return Array.isArray(messages) &&
           messages.every(m => isMessage(m));
}
