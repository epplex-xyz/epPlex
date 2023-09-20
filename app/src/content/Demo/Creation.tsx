import React from "react";
import Box from "@mui/material/Box";
import { TextDivider } from "@components/Divider/TextDivider";
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { DatePicker } from '@mui/x-date-pickers/DatePicker';
import { TimePicker } from '@mui/x-date-pickers/TimePicker';
import dayjs from "dayjs";
import { styled, Theme } from '@mui/material';
import { TextField } from "@mui/material";

const WalletMenuItem = styled(DatePicker)(({ theme }: { theme: Theme }) => ({
    popper: {
        // "&. MuiPaper-root-MuiPickersPopper-paper": {
        "&. MuiButtonBase-root-MuiPickersDay-root": {
            color: "black"
        },
        // }
    }
}));

export function Creation() {
    return (
        <Box
            component="div"
            position="relative"
            height="100%"
            flexDirection="column"
            rowGap={"16px"}
            width={"300px"}
        >
            <TextDivider>Ephemerality</TextDivider>

            <div className={"flex flex-row"}>
                <LocalizationProvider dateAdapter={AdapterDayjs}>
                    <DatePicker
                        label=""
                        format="YYYY-MM-DD"
                        slotProps={{
                            textField: {
                                size: "small"
                            },
                            day: {
                                sx: {
                                    ['&[data-mui-date="true"] .Mui-selected']: {
                                        // Reset the background color of the selected date
                                        backgroundColor: "blue"
                                    },
                                    ":not(.Mui-selected)": {
                                        backgroundColor: "#fff",
                                        color: "black",
                                        borderColor: "red"
                                    },
                                    "&.MuiPickersDay-root.Mui-selected": {
                                        color: "#fff",
                                        backgroundColor: "red",
                                        borderColor: "red",
                                        ":hover": {
                                            color: "#fff",
                                            backgroundColor: "red",
                                            borderColor: "red"
                                        }
                                    },
                                    ":hover": {
                                        color: "#fff",
                                        backgroundColor: "red",
                                        borderColor: "red"
                                    }
                                }
                            }
                        }}
                    />
                    <DatePicker
                        slotProps={{
                            popper: {
                                // sx: {
                                //     "&. MuiButtonBase-root-MuiPickersDay-root": {
                                //         color: "black"
                                //     },
                                // }
                                color: "black"
                            },
                            // textField: {
                            //     sx: {color: "black"}
                            // }
                            textField: {
                                classes: {
                                    root: "inputPadding"
                                }
                            },
                            day: {
                                sx: {
                                    "&. MuiButtonBase-root-MuiPickersDay-root": {
                                        backgroundColor: "red",
                                        color: "black"
                                    },
                                }
                            }
                        }}



                        // PopperProps={{
                        //     sx: {
                        //         "& .MuiTypography-root": {
                        //             color: "black",
                        //         },
                        //     },
                        // }}
                        label="Date"

                        // renderInput={(params) => (
                        //     <TextField
                        //         {...params}
                        //         InputProps={{
                        //             ...params.InputProps,
                        //             classes: {
                        //                 ...params.InputProps.classes,
                        //                 popper: {color: "black",
                        //             },
                        //         }}
                        //     />
                        // )}
                        // PopperProps={{
                        //     sx: {"& .MuiPickersDay-root": {border: '4px solid red'}},
                        // }}

                        // sx={{
                        //     "& .MuiTypography-root ": {
                        //         color: "black",
                        //     },
                        // }}

                    />
                    <TimePicker
                        label="Time"
                        defaultValue={dayjs(("00:00:00"), "HH:mm:ss")}
                    />
                </LocalizationProvider>
            </div>

            <TextDivider>Image</TextDivider>

            <TextDivider>Details</TextDivider>
        </Box>
    );
}
