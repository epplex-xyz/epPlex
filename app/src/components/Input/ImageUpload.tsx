import React, { useState } from 'react';
import Dropzone from 'react-dropzone';
import Box from "@mui/material/Box";
import { Text } from "@components/Text/TextComponent";
import style from "../../styles/style.module.scss";

export function ImageUpload(initialFile: File | null): {
    component: React.ReactNode,
    selectedFile: File | null,
    temporaryUrl: string
} {
    const [selectedFile, setSelectedFile] = useState<File | null>(initialFile);
    const url = selectedFile === null ? "" : URL.createObjectURL(selectedFile);


    const component =
        <Dropzone onDrop={(acceptedFiles) => setSelectedFile(acceptedFiles[0])}>
            { ({ getRootProps, getInputProps }) => (
                <Box
                    component={"div"}
                    sx={{
                        borderRadius: style.borderRadiusMd,
                        boxShadow: (theme) => `inset 0 0 0 1px ${theme.palette.text.primary}`,
                    }}
                    display={"flex"}
                    flexDirection={"column"}
                    alignItems={"center"}
                    justifyContent={"center"}
                    height={"150px"}
                    width={"150px"}
                    textAlign={"center"}
                    {...getRootProps()}
                >
                    {selectedFile === null ?
                        <>
                            <input {...getInputProps()} />
                            <Text.Subtitle1>
                                Drag & drop <br/>
                                or Click to select
                            </Text.Subtitle1>
                            <Text.Body2>(.png or.mp4)</Text.Body2>
                        </>
                        : <img
                            style={{
                                width:"inherit",
                                height:"inherit",
                                borderRadius: style.borderRadiusMd,
                            }}
                            src={url}
                            alt="Selected"
                        />
                    }
                </Box>
            )}
        </Dropzone>;

    return {
        component,
        selectedFile,
        temporaryUrl: url
    };
}