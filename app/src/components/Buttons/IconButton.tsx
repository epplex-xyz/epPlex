import { IconButton } from "@mui/material";
import { paletteLight } from "@styles/palette";

export function BWIconButton({children, ...props}) {
    return (
        <IconButton
            sx={{
                backgroundColor: "secondary.main",
                color: "primary.main",
                '&:hover': {
                    backgroundColor: `${paletteLight.secondary.main}` + "80"
                },
            }}
            {...props}
        >
            {children}
        </IconButton>
    );
}