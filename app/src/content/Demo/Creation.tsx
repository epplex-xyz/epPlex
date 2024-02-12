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
import CircularProgress from "@mui/material/CircularProgress";
import { ToastText } from "@components/Text/ToastText";
import {useAnchorWallet} from "@solana/wallet-adapter-react";

export function Creation() {
    const {dateComponent, date} = MyDatePicker({width: "150px"});
    const {timeComponent, time} = MyTimePicker({width: "150px"});
    const nameInput = StandardInput({placeholder: "Name"});
    const symbolInput = StandardInput({placeholder: "Symbol"});
    const traitInput = StandardInput(
        {
            placeholder: '[{"trait_type": "background", "value": "blue"}]',
            height: "100px",
            multiline: true
        }
    );

    const imageUpload = ImageUpload(null);
    const {program, hasCreatedtState: {setHasCreated}} = useProgramApis();
    const wallet = useAnchorWallet();
    const combinedDate = combineDateAndTime(date!.toDate(), time!.toDate());
    const unixTime = Math.floor(combinedDate.getTime() / 1000);
    const [loading, setLoading] = React.useState(false);

    // const testButton = useCallback(async () => {
    //     // const txId = await program.createProgramDelegate();
    //     const txId = await program.renewToken();
    // }, []);

    const handleCreate = useCallback(async () => {
        setLoading(true);
        try {
            const current = Math.floor((new Date()).getTime() / 1000);
            const offset = unixTime - current;

            if (offset < 0) {
                throw new Error("Invalid time");
            }

            if (imageUpload.selectedFile === null) {
                throw new Error("No image uploaded");
            }

            if (symbolInput.input === "") {
                throw new Error("Symbol not specified");
            }

            if (nameInput.input === "") {
                throw new Error("Name not specified");
            }

            const traitObjects = JSON.parse(traitInput.input);
            if (validateTraits(traitObjects)) {
                throw new Error("Invalid traits");
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
            const metadata = makeJson(imageUrl, nameInput.input, symbolInput.input, wallet!.publicKey, traitObjects);
            const metadataName = addExtension(fileName, "json");

            // Upload metadata
            const metadataRes = await fetch("/api/upload", {
                method: 'POST', // Use the POST method
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    name: metadataName,
                    fileBuffer: Buffer.from(JSON.stringify(metadata)),
                }),
            }).then((res) => res.json());

            if (metadataRes.ok) {
                throw new Error("Failed to upload metadata");
            }

            // const txId = await program.createToken(
            //     offset,
            //     nameInput.input,
            //     symbolInput.input,
            //     metadataRes.message, //metadata uri
            // );
            const txId = "";

            if (txId === "") {
                throw new Error("Failed to create epNFT");
            } else {
                setHasCreated((prev) => !prev);
                toast.success(
                    <ToastText text={"Successfully created epNFT:"} signature={txId}/>
                );
            }
        } catch (e: any) {
            console.log("Failed creating epNFT", e);
            toast.error(e.message);
        } finally {
            setLoading(false);
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
            rowGap={"16px"}
            width={{ sm: "300px", md: "400px" }}
        >
            <TextDivider>Ephemerality</TextDivider>
            <div className="flex flex-row justify-evenly">
                {dateComponent}

                {timeComponent}
            </div>
            <div className="flex justify-center">
                <Text.Body1>
                    Self-destruct in &nbsp;<Timer endTimestamp={unixTime}/>
                </Text.Body1>
            </div>

            <TextDivider>Image</TextDivider>
            <div className="flex w-full justify-center">
                {imageUpload.component}
            </div>


            <TextDivider>Details</TextDivider>
            <div className="flex flex-col items-center px-2 gap-y-3 mx-4">
                {/* Name */}
                <div className="flex justify-between w-full items-center">
                    <Text.Body1>
                        Name
                    </Text.Body1>
                    {nameInput.inputComponent}
                </div>

                {/* Symbol */}
                <div className="flex justify-between w-full items-center">
                    <Text.Body1>
                        Symbol
                    </Text.Body1>
                    {symbolInput.inputComponent}
                </div>

                {/* Traits */}
                <div className={"flex w-full flex-col gap-y-2"}>
                    <Text.Body1>
                        Traits
                    </Text.Body1>
                    {traitInput.inputComponent}
                </div>
            </div>


            {/* Button */}
            <div className={"flex w-full justify-center"}>
                <Button
                    variant={"contained"}
                    sx={{
                        marginTop: "16px",
                        columnGap: "8px",
                    }}
                    onClick={handleCreate}
                >
                    {loading ?
                        <>
                            Creating <CircularProgress  size={"14px"} sx={{color: "text.secondary"}}/>
                        </>
                        : <>
                            Create epNFT
                        </>
                    }
                </Button>
            </div>
        </Box>
    );
}
