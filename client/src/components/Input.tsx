import * as React from "react";

const Input = ({ handleChange, value, type }: any) => {
    return (
        <input
            type={ type }
            value={ value }
            onChange={ e => handleChange(e.target.value) }/>
    );
};

export default Input;
