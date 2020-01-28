import * as React from "react";

const TextArea = ({ handleChange, value }: any) => {
    return (
        <textarea
            value={ value }
            onChange={ e => handleChange(e.target.value) } />
    );
};

export default TextArea;
