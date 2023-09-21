import React, { useState } from 'react';
import axios from 'axios';
import Dropzone from 'react-dropzone';
import Box from "@mui/material/Box";
import { Text } from "@components/Text/TextComponent";

export function ImageUpload() {
    const [selectedFile, setSelectedFile] = useState<any>(null);

    const handleFileUpload = async (file) => {
        // Optional: Resize and compress the image
        // const resizedFile = await resizeFile(file); // Adjust dimensions as needed

        // Create a FormData object and append the file
        const formData = new FormData();
        // formData.append('image', resizedFile);

        // Send the file to the server (you should have a server endpoint to handle uploads)
        try {
            const response = await axios.post('/upload', formData, {
                headers: {
                    'Content-Type': 'multipart/form-data',
                },
            });

            // Handle the response from the server (e.g., show a success message)
            console.log('Image upload response:', response.data);
        } catch (error) {
            // Handle errors (e.g., display an error message)
            console.error('Image upload error:', error);
        }
    };

    return (
        <Box
            component={"div"}
            sx={{
                borderRadius: "12px",
                boxShadow: (theme) => `inset 0 0 0 2px ${theme.palette.text.primary}`,
            }}
            display={"flex"}
            flexDirection={"column"}
            alignItems={"center"}
        >
            <Dropzone onDrop={(acceptedFiles) => setSelectedFile(acceptedFiles[0])}>
                {({ getRootProps, getInputProps }) => (
                    <div{...getRootProps()}>
                        <input {...getInputProps()} />
                        <Text.H6>
                            Drag & drop an image
                        </Text.H6>
                        <Text.H6>
                            or
                        </Text.H6>
                        <Text.H6>
                            Click to select one
                        </Text.H6>

                    </div>
                )}
            </Dropzone>
            {selectedFile && (
                <div>
                    <img src={URL.createObjectURL(selectedFile)} alt="Selected" />
                    {/*<Button variant="contained" onClick={() => handleFileUpload(selectedFile)}>Upload</Button>*/}
                </div>
            )}
        </Box>
    );
}