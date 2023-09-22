import { Toaster } from "react-hot-toast";
import React from "react";
import { useTheme } from "@mui/material/styles";

export function MyToaster() {
    const theme = useTheme();
    return (
        <Toaster
            position="bottom-left"
            reverseOrder={false}
            toastOptions={{
                duration: 5000,
                style: {background: theme.palette.secondary.main},
                iconTheme: {primary: theme.palette.primary.main, secondary: theme.palette.secondary.main}
            }}
        />
    );
}