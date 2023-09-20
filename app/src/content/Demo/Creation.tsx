import React from "react";
import Box from "@mui/material/Box";
import { TextDivider } from "@components/Divider/TextDivider";
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { TimePicker } from '@mui/x-date-pickers/TimePicker';
import dayjs from "dayjs";
import { MyDatePicker } from "@components/Input/DatePicker";



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
                    <MyDatePicker/>

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
