import React, { useCallback } from "react";
import Box from "@mui/material/Box";
import { TextDivider } from "@components/Divider/TextDivider";
import { MyDatePicker } from "@components/Input/DatePicker";
import { MyTimePicker } from "@components/Input/TimePicker";
import { ImageUpload } from "@components/Input/ImageUpload";
import { Timer } from "@components/Text/Timer";
import { Text } from "@components/Text/TextComponent";
import { StandardInput } from "@components/Input/TextField";
import { addExtension, combineDateAndTime, validateTraits } from "../../../utils/general";
import Button from "@mui/material/Button";
import { useProgramApis } from "../../providers/ProgramApisProvider";
import { Keypair } from "@solana/web3.js";
import toast from "react-hot-toast";
import { makeJson } from "../../../utils/metadata";

export function Creation() {
    const {dateComponent, date} = MyDatePicker({width: "150px"});
    const {timeComponent, time} = MyTimePicker({width: "150px"});
    const nameInput = StandardInput({placeholder: "Name"});
    const symbolInput = StandardInput({placeholder: "Symbol"});
    const traitInput = StandardInput(
        {
            placeholder: '[{"trait_type": "background", "value": "blue"}]',
            width: "100%",
            height: "200px"
        }
    );
    const imageUpload = ImageUpload();
    const {program} = useProgramApis();
    const combinedDate = combineDateAndTime(date!.toDate(), time!.toDate());
    const unixTime = Math.floor(combinedDate.getTime() / 1000);

    const handleCreate = useCallback(async () => {
        try {
            const current = Math.floor((new Date()).getTime() / 1000);
            const offset = unixTime - current;

            if (offset < 0) {
                throw new Error("Invalid time");
            }

            // Image upload
            const fileData = await imageUpload.selectedFile.arrayBuffer();
            const fileName = imageUpload.selectedFile.name;
            const imageRes = await fetch("/api/upload", {
                method: 'POST', // Use the POST method
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    name: fileName,
                    fileBuffer: Buffer.from(fileData),
                }),
            }).then((res) => res.json());

            if (imageRes.ok) {
                throw new Error("Failed to upload image");
            }
            const imageUrl = imageRes.message;


            const traitObjects = JSON.parse(traitInput.input);
            if (validateTraits(traitObjects)) {
                throw new Error("Invalid traits");
            }

            console.log("traits", traitObjects);
            const metadata = makeJson(imageUrl, nameInput.input, symbolInput.input, program.wallet.publicKey, traitObjects);
            const metadataName = addExtension(fileName, "json");

            const metadataRes = await fetch("/api/upload", {
                method: 'POST', // Use the POST method
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    name: metadataName,
                    fileBuffer: Buffer.from(JSON.stringify(metadata)),
                }),
            }).then((res) => res.json());

            if (metadataRes.ok) {
                throw new Error("Failed to upload image");
            }

            const metadataUri = metadataRes.message;
            console.log("metadata", metadataUri);

            const mint = Keypair.generate();
            await program.createToken(
                mint,
                offset,
                nameInput.input,
                symbolInput.input,
                metadataUri,
            );

        } catch (e: any) {
            console.log("Failed creating epNFT", e);
            toast.error(e.message);
        }

    }, [
        unixTime,
        nameInput.input,
        symbolInput.input,
        traitInput.input,
        imageUpload.selectedFile
    ]);

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

            {imageUpload.component}

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

                    {traitInput.inputComponent}
                </div>
            </div>


            <Button
                variant={"contained"}
                sx={{
                    marginTop: "16px"
                }}
                onClick={handleCreate}
            >
                <Text.H6 color={"primary.main"}>
                    Create epNFT
                </Text.H6>
            </Button>
        </Box>
    );
}
