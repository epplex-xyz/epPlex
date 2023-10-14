import {styled} from "@mui/material/styles";
import TextField, { TextFieldProps } from "@mui/material/TextField";
import React, { useState } from "react";
import style from "../../styles/style.module.scss";

const Input = styled(TextField)(({theme }) => ({
    '& .MuiTextField-root': {
        display: 'contents',
    },
    '& .MuiInputBase-input': {
        padding: '0px',
        color: theme.palette.secondary.main,
        textAlign: "center",
        fontSize: "14px"
    },
    '& .MuiOutlinedInput-root': {
        // backgroundColor: theme.palette.secondary.main,
        color: theme.palette.secondary.main,
        borderWidth: '0px',
        boxShadow: `inset 0 0 0 1px ${theme.palette.text.primary}`,
        borderRadius: style.borderRadiusMd,
        padding: "8px 16px",
    },
    "& .MuiOutlinedInput-notchedOutline": {
        border: "none"
    }
}));

interface Props  {
    height?: string;
}

export function StandardInput({
    height = "undefined",
    ...props
}: Props & TextFieldProps): {inputComponent: React.ReactNode, input: string} {
    const [input, setInput] = useState("");

    const handleSearchChange = (e) => {
        setInput(e.target.value);// }
    };

    const inputComponent = <Input
        autoComplete={"off"}
        value={input}
        placeholder={props.placeholder}
        multiline={props.multiline}
        type={"text"}
        onChange={handleSearchChange}
        sx={{
            borderRadius: style.borderRadiusMd,
            textAlign: "center",
            '& .MuiOutlinedInput-root': {
                height: height,
            },
            ...props,
        }}
        // inputProps={{
        //     autocomplete: 'chrome-off',
        //     form: {
        //         autocomplete: 'chrome-off',
        //     },
        // }}
    />;

    return {inputComponent, input};
}