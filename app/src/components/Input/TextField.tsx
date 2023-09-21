import {styled} from "@mui/material/styles";
import TextField from "@mui/material/TextField";
import React, { useState } from "react";
export const StyledTextField = styled(TextField)(({theme }) => ({
    '& .MuiTextField-root': {
        display: 'contents',
    },
    '& .MuiInputBase-input': {
        padding: '0px',
        maxWidth: '30px',
        height: "30px",
        fontSize: "1.2rem",
        color: theme.palette.secondary.main,
        fontWeight: "900",
        [theme.breakpoints.down('sm')]: {
            maxWidth: '25px',
            height: "25px",
            fontSize: "1rem"
        },
    },
    '& .MuiOutlinedInput-root': {
        backgroundColor: theme.palette.secondary.main,
        borderWidth: '1px'
    },
    // could also do at the textfield
    // sx={{ "& .MuiOutlinedInput-notchedOutline": { border: "none" } }}
    "& .MuiOutlinedInput-notchedOutline": {
        border: "none"
    }
}));

export const StyledSearchInput = styled(TextField)(({theme }) => ({
    // width: "80%",
    // paddingLeft: "64px",
    '& .MuiTextField-root': {
        display: 'contents',
    },
    '& .MuiInputBase-input': {
        padding: '0px',
        maxWidth: "200px",
        color: theme.palette.primary.main,
        textAlign: "center",
    },
    '& .MuiOutlinedInput-root': {
        backgroundColor: theme.palette.secondary.main,
        color: theme.palette.secondary.main,
        borderWidth: '0px'
    },
    // could also do at the textfield
    // sx={{ "& .MuiOutlinedInput-notchedOutline": { border: "none" } }}
    "& .MuiOutlinedInput-notchedOutline": {
        border: "none"
    }
}));

export function StandardInput() {
    const [input, setInput] = useState("");

    const handleSearchChange = (e) => {
        setInput(e.target.value);// }
    };

    const inputComponent = <StyledSearchInput
        autoComplete="off"
        value={input}
        onChange={handleSearchChange}
        sx={{ borderRadius: "12px", padding: "4px 8px"}}
    />;

    return {inputComponent, input};
}