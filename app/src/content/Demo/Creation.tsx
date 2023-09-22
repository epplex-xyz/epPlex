import React from "react";
import Box from "@mui/material/Box";
import { TextDivider } from "@components/Divider/TextDivider";
import { MyDatePicker } from "@components/Input/DatePicker";
import { MyTimePicker } from "@components/Input/TimePicker";
import { ImageUpload } from "@components/Input/ImageUpload";
import { Timer } from "@components/Text/Timer";
import { Text } from "@components/Text/TextComponent";
import { StandardInput } from "@components/Input/TextField";
import { combineDateAndTime } from "../../utils/general";
import Button from "@mui/material/Button";
import { TraitInputField } from "./TraitInput";


export function Creation() {
    const {dateComponent, date} = MyDatePicker({width: "150px"});
    const {timeComponent, time} = MyTimePicker({width: "150px"});
    const nameInput = StandardInput({placeholder: "Name"});
    const symbolInput = StandardInput({placeholder: "Symbol"});

    const combinedDate = combineDateAndTime(date!.toDate(), time!.toDate());
    const unixTime = Math.floor(combinedDate.getTime() / 1000);


    const handleCreate = () => {
        //destroyTimestamp
        //Image
        //name
        // symbol
        // traits - do validation

        // adding side padding
    }

    return (
        <Box
            component="div"
            position="relative"
            height="100%"
            display={"flex"}
            flexDirection="column"
            rowGap={"24px"}
            width={{ sm: "300px", md: "400px" }}

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

            <div className="flex flex-col items-center px-2 gap-y-2">
                <div className="flex justify-between w-full">
                    <Text.H6>
                        Name
                    </Text.H6>
                    {nameInput.inputComponent}
                </div>
                <div className="flex justify-between w-full">
                    <Text.H6>
                        Symbol
                    </Text.H6>
                    {symbolInput.inputComponent}
                </div>
                <div className={"w-full"}>
                    <Text.H6>
                        Traits
                    </Text.H6>

                    <TraitInputField/>
                </div>
            </div>


            <Button
                variant={"contained"}
                sx={{
                    marginTop: "16px"
                }}
            >
                <Text.H6 color={"primary.main"}>
                    Create epNFT
                </Text.H6>
            </Button>
        </Box>
    );
}
