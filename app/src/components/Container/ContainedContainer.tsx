import Box from "@mui/material/Box";
import style from "../../styles/style.module.scss";

export function ContainedContainer({children}) {
    return (
        <Box
            component="div"
            position="relative"
            rowGap={"16px"}
            display={"flex"}
            alignSelf={"center"}
            // color={"primary.main"}
            // bgcolor={"secondary.main"}
            sx={{
                borderRadius: style.borderRadiusMd,
                boxShadow: (theme) => `inset 0 0 0 1px ${theme.palette.text.primary}`,
            }}
            padding={"8px 16px"}
        >
            {children}
        </Box>
    );
}