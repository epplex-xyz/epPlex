import { Text } from "@components/Text/TextComponent";
import Box from "@mui/material/Box";

export function TextDivider({children}) {
    return (
        <div className="flex items-center justify-center self-stretch gap-x-2">
            <Text.H6 className={"uppercase"}>
                {children}
            </Text.H6>
            <Box
                component={"div"}
                height={"1px"}
                bgcolor={"background.paper"}
                width={"100%"}
            />
        </div>
    );
}