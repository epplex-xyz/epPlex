import { ButtonLinkProps } from "src/components/Buttons/LinkButton";
import { Text } from "src/components/Text/TextComponent";

type ButtonConfigKeys = "demo" | "docs";

export const ButtonConfig: Record<ButtonConfigKeys, ButtonLinkProps> = {
    demo: {
        variant: "contained",
        href: "/demo",
        children: <Text.Body1 color={"text.secondary"}>
            Try Demo
        </Text.Body1>,
        sx: {
            paddingX: '32px',
        }
    },
    docs: {
        variant: "outlined",
        href: "https://twitter.com/epplex_xyz",
        children: <Text.Body1 color={"text.primary"} >
            Stay Updated
        </Text.Body1>,
        sx: {
            paddingX: '32px',
        },
        linkType: "external",
        blank: true
    },
};