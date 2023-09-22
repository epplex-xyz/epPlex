import { Text } from "@components/Text/TextComponent";
import Box from "@mui/material/Box";

export function TextDivider({children, uppercase = true}) {
    return (
        <div className="flex items-center justify-center self-stretch gap-x-2">
            <Text.H6 className={`whitespace-nowrap ${uppercase ? "uppercase" : undefined}`}>
                {children}
            </Text.H6>
            <Box
                component={"div"}
                height={"3px"}
                bgcolor={"background.paper"}
                width={"100%"}
            />
        </div>
    );
}