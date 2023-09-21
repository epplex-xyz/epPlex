// Taken from here https://stackoverflow.com/questions/76767152/i-am-using-react-mui-mui-x-date-pickers-please-tell-me-how-to-change-color-of
import React from "react";
import { DatePicker } from '@mui/x-date-pickers/DatePicker';

export function MyDatePicker() {
    return (
        <DatePicker
            label=""
            format="YYYY-MM-DD"
            sx={{
                color: "red",
                "&. MuiTypography-root-MuiDayCalendar-weekDayLabel": {
                    color: "red"
                },
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
                        // color: "text.seoncdary",
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
                        // ":hover": {
                        //     color: "text.primary",
                        //     backgroundColor: "text.secondary",
                        // },
                        // "& .MuiPickersYear-yearButton": {
                        //     color: "black"
                        // }
                        "&.MuiDayCalendar-weekDayLabel": {
                            color: "red"
                        },
                        "&.MuiDayCalendar-root": {
                            color: "red"
                        },
                        "&. MuiTypography-root": {
                            color: "red"
                        },
                        ['&[data-mui-day="true"] .Mui-selected']: {
                            color: "red"
                        }
                    }
                },
                popper: {
                    sx: {
                        "& .MuiPickersYear-yearButton": {
                            color: "black"
                        },
                        "&. MuiTypography-root-MuiDayCalendar-weekDayLabel": {
                            color: "black"
                        },
                        "&. MuiDayCalendar-weekDayLabel": {
                            color: "black"
                        },
                        "&. MuiTypography-root": {
                            color: "black"
                        }
                    }
                },
                calendarHeader: {
                    sx: {
                        "& .MuiPickersYear-yearButton": {
                            color: "black"
                        },
                        "&. MuiTypography-root-MuiDayCalendar-weekDayLabel": {
                            color: "black"
                        },
                        "&. MuiDayCalendar-weekDayLabel": {
                            color: "black"
                        },
                        "&. MuiTypography-root.Mui-selected": {
                            color: "black"
                        },
                    }
                },
                desktopPaper: {
                    sx: {
                        "& .MuiPickersYear-yearButton": {
                            color: "black"
                        },
                        "&. MuiTypography-root-MuiDayCalendar-weekDayLabel": {
                            color: "black"
                        },
                        "&. MuiDayCalendar-weekDayLabel": {
                            color: "black"
                        },
                        "&. MuiTypography-root.Mui-selected": {
                            color: "black"
                        }
                    }
                },
                layout: {
                    sx: {
                        "& .MuiPickersYear-yearButton": {
                            color: "black"
                        },
                        "&. MuiTypography-root-MuiDayCalendar-weekDayLabel": {
                            color: "black"
                        },
                        "&. MuiDayCalendar-weekDayLabel": {
                            color: "black"
                        },
                        "&. MuiTypography-root.Mui-selected": {
                            color: "black"
                        }
                    }
                },

                actionBar: {
                    sx: {
                        "& .MuiPickersYear-yearButton": {
                            color: "black"
                        },
                        "&. MuiTypography-root-MuiDayCalendar-weekDayLabel": {
                            color: "black"
                        },
                        "&. MuiDayCalendar-weekDayLabel": {
                            color: "black"
                        },
                        "&. MuiTypography-root": {
                            color: "black"
                        }
                    }
                },
                textField: {
                    InputProps: {
                        sx: {
                            "& .MuiPickersYear-yearButton": {
                                color: "red"
                            },
                            "&. MuiTypography-root-MuiDayCalendar-weekDayLabel": {
                                color: "red"
                            },
                            "&. MuiDayCalendar-weekDayLabel": {
                                color: "red"
                            },
                            "&. MuiTypography-root.Mui-selected": {
                                color: "red"
                            },
                            "&. MuiInputBase-root": {
                                color: "white"
                            },
                            "&. MuiTextField-root": {
                                color: "white !important"
                            },
                            "&. MuiInputBase-root-MuiOutlinedInput-root": {
                                color: "white !important"
                            },
                            "&. MuiOutlinedInput-root": {
                                color: "white !important"
                            },

                            "&. MuiInputBase-input-MuiOutlinedInput-input": {
                                color: "white !important"
                            }
                        }
                    },
                    sx: {
                        "& .MuiPickersYear-yearButton": {
                            color: "red"
                        },
                        "&. MuiTypography-root-MuiDayCalendar-weekDayLabel": {
                            color: "red"
                        },
                        "&. MuiDayCalendar-weekDayLabel": {
                            color: "red"
                        },
                        "&. MuiTypography-root.Mui-selected": {
                            color: "red"
                        },
                        "&. MuiInputBase-root": {
                            color: "white"
                        },
                        "&. MuiTextField-root": {
                            color: "white !important"
                        },
                        "&. MuiInputBase-root-MuiOutlinedInput-root": {
                            color: "white !important"
                        },
                        "&. MuiOutlinedInput-root": {
                            color: "white !important"
                        },

                        "&. MuiInputBase-input-MuiOutlinedInput-input": {
                            color: "white !important"
                        }
                    }
                },
            }}
        />

    );
}
