// Taken from here https://stackoverflow.com/questions/76767152/i-am-using-react-mui-mui-x-date-pickers-please-tell-me-how-to-change-color-of
import React from "react";
import { AdapterDayjs } from "@mui/x-date-pickers/AdapterDayjs";
import {TextField } from "@mui/material";
import { LocalizationProvider } from "@mui/x-date-pickers/LocalizationProvider";
import dayjs, { Dayjs } from "dayjs";
import { TimePicker } from "@mui/x-date-pickers/TimePicker";

export function MyTimePicker({width}) {
    const [time, setTime] = React.useState<Dayjs | null>(
        dayjs(("00:00:00"), "HH:mm:ss"),
    );
    const handleChange = (newValue: Dayjs | null) => {
        setTime(newValue);
    };

    const timeComponent = <LocalizationProvider dateAdapter={AdapterDayjs}>
        <TimePicker
            value={time}
            InputProps={{
                sx: {
                    "& .MuiIconButton-root": {
                        color: "text.primary"
                    },
                    "&. MuiInputLabel-root": {
                        color: "text.primary",
                        "&.Mui-selected": {
                            color: "text.primary",
                        },
                    }
                }
            }}
            onChange={handleChange}
            renderInput={(params) =>
                <TextField
                    {...params}
                    sx={{
                        '& .MuiOutlinedInput-root': {
                            '& fieldset': {
                                borderColor: 'text.primary',
                            },
                            '&:hover fieldset': {
                                borderColor:'text.primary',
                            },
                            '&.Mui-focused fieldset': {
                                borderColor: 'text.primary',
                            },
                        },
                        "&. MuiInputLabel-root": {
                            color: "text.primary"
                        },
                        "&. MuiFormLabel-root": {
                            color: "text.primary"
                        },
                        width: width
                    }}
                    InputLabelProps={{
                        sx: {
                            color: "text.primary",
                            "&.Mui-focused": {
                                color: "text.primary"
                            }
                        }
                    }}
                />
            }
            label="Time"
            PaperProps={{
                sx: {
                    "& .MuiClockNumber-root": {
                        color: "text.secondary",
                        "&.Mui-selected": {
                            color: "text.primary",
                        },
                    }
                }
            }}
        />
    </LocalizationProvider>;

    return ({timeComponent, time});
}
