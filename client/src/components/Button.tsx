import * as React from "react";

function Button({ handleClick, payload, value }: any) {
    return (
        <div>
            <button
                onClick={
                    () => { handleClick(payload); }
                }>{ value }</button>
        </div>
    );
}

export default Button;
