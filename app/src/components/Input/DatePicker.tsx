// Taken from here https://stackoverflow.com/questions/76767152/i-am-using-react-mui-mui-x-date-pickers-please-tell-me-how-to-change-color-of
import React from "react";
import { DatePicker } from '@mui/x-date-pickers/DatePicker';
import { styled, Theme } from "@mui/material";

export function MyDatePicker() {
    return (
        <DatePicker
            label=""
            format="YYYY-MM-DD"
            sx={{
                color: "text.secondary"
            }}
            slotProps={{
                day: {
                    sx: {
                        // ['&[data-mui-date="true"] .Mui-selected']: {
                        //     // Reset the background color of the selected date
                        //     backgroundColor: "blue"
                        // },
                        // ":not(.Mui-selected)": {
                        //     // backgroundColor: "#fff",
                        //     color: "text.secondary",
                        //     // borderColor: "red"
                        // },
                        color: "text.secondary",
                        // "&.MuiPickersDay-root.Mui-selected": {
                        //     color: "#fff",
                        //     backgroundColor: "red",
                        //     borderColor: "red",
                        //     // ":hover": {
                        //     //     color: "#fff",
                        //     //     backgroundColor: "red",
                        //     //     borderColor: "red"
                        //     // }
                        // },
                        ":hover": {
                            color: "text.primary",
                            backgroundColor: "text.secondary",
                        },
                        "& .MuiPickersYear-yearButton": {
                            color: "black"
                        }
                        // sx: {
                        //     color: "text.secondary"
                        // }
                        // sx: {
                        //     color: "text.secondary"
                        // }
                    }
                },
                // year: {
                //     sx: {
                //         color: "text.secondary"
                //     }
                // },
                calendarHeader: {
                    sx: {
                        color: "text.secondary",
                        "& .MuiPickersYear-yearButton": {
                            color: "black"
                        }
                    }
                }
            }}
        />

    );
}
