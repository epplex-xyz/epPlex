import {styled} from "@mui/material/styles";
import TextField from "@mui/material/TextField";
import React, { useState } from "react";
import style from "../../styles/style.module.scss";


const Input = styled(TextField)(({theme }) => ({
    '& .MuiTextField-root': {
        display: 'contents',
    },
    '& .MuiInputBase-input': {
        padding: '0px',
        // maxWidth: "200px",
        color: theme.palette.secondary.main,
        textAlign: "center",
    },
    '& .MuiOutlinedInput-root': {
        // backgroundColor: theme.palette.secondary.main,
        color: theme.palette.secondary.main,
        borderWidth: '0px',
        boxShadow: `inset 0 0 0 1px ${theme.palette.text.primary}`
    },
    // could also do at the textfield
    // sx={{ "& .MuiOutlinedInput-notchedOutline": { border: "none" } }}
    "& .MuiOutlinedInput-notchedOutline": {
        border: "none"
    }
}));

export function StandardInput({placeHolder, textAlign = "center"}) {
    const [input, setInput] = useState("");

    const handleSearchChange = (e) => {
        setInput(e.target.value);// }
    };

    const inputComponent = <Input
        autoComplete="off"
        value={input}
        placeholder={placeHolder}
        onChange={handleSearchChange}
        sx={{
            borderRadius: style.borderRadiusMd,
            textAlign: textAlign
        }}
    />;

    return {inputComponent, input};
}