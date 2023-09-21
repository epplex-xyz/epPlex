import React from "react";
import Box from "@mui/material/Box";
import { TextDivider } from "@components/Divider/TextDivider";
import { MyDatePicker } from "@components/Input/DatePicker";
import { MyTimePicker } from "@components/Input/TimePicker";
import { ImageUpload } from "@components/Input/ImageUpload";
import { Timer } from "@components/Text/Timer";
import { Text } from "@components/Text/TextComponent";
import { StandardInput, StyledSearchInput, StyledTextField } from "@components/Input/TextField";


function combineDateAndTime(date: Date, time: Date) {
    const year = date.getFullYear();
    const month = date.getMonth();
    const day = date.getDate();

    const hours = time.getHours();
    const minutes = time.getMinutes();
    const seconds = time.getSeconds();

    return new Date(year, month, day, hours, minutes, seconds);
}

export function Creation() {
    const {dateComponent, date} = MyDatePicker({width: "150px"});
    const {timeComponent, time} = MyTimePicker({width: "150px"});
    const {inputComponent, input} = StandardInput();

    const combinedDate = combineDateAndTime(date!.toDate(), time!.toDate());
    const unixTime = Math.floor(combinedDate.getTime() / 1000);

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
            <div className="flex justify-center">
                <Text.H6>
                    Self-destruct in &nbsp;<Timer endTimestamp={unixTime}/>
                </Text.H6>
            </div>

            <TextDivider>Image</TextDivider>

            <ImageUpload/>
            <TextDivider>Details</TextDivider>

            <div className="justify-between flex">
                <Text.H6>
                    Name
                </Text.H6>
                {inputComponent}
            </div>
            <div className="justify-between flex">
                <Text.H6>
                    Symbol
                </Text.H6>
                {inputComponent}
            </div>

        </Box>
    );
}
