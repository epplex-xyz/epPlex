import React from "react";
import Box from "@mui/material/Box";
import { TextDivider } from "@components/Divider/TextDivider";
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { TimePicker as Time } from '@mui/x-date-pickers/TimePicker';
import dayjs from "dayjs";


import { TimePicker } from 'antd';
import { TextField } from "@mui/material";
import { MyDatePicker } from "@components/Input/DatePicker";
import { MyTimePicker } from "@components/Input/TimePicker";


export function Creation() {
    const {dateComponent, date} = MyDatePicker({width: "150px"});
    const {timeComponent, time} = MyTimePicker({width: "150px"});;

    return (
        <Box
            component="div"
            position="relative"
            height="100%"
            display={"flex"}
            flexDirection="column"
            rowGap={"16px"}
            width={"400px"}
        >
            <TextDivider>Ephemerality</TextDivider>

            <div className="flex flex-row justify-evenly">
                {dateComponent}

                {timeComponent}
            </div>


            <div className={"flex flex-row"}>

            </div>

            <TextDivider>Image</TextDivider>

            <TextDivider>Details</TextDivider>
        </Box>
    );
}
