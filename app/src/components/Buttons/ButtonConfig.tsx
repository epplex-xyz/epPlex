import { ButtonLinkProps } from "src/components/Buttons/LinkButton";
import { Text } from "src/components/Text/TextComponent";

type ButtonConfigKeys = "demo" | "docs";

export const ButtonConfig: Record<ButtonConfigKeys, ButtonLinkProps> = {
    demo: {
        variant: "contained",
        href: "/demo",
        children: <Text.H5 color={"text.secondary"}>
            Try Demo
        </Text.H5>,
        sx: {
            paddingX: '32px',
        }
    },
    docs: {
        variant: "outlined",
        href: "https://twitter.com/epplex_xyz",
        children: <Text.H5>
            Stay Updated
        </Text.H5>,
        sx: {
            paddingX: '32px',
        },
        linkType: "external",
        blank: true
    },
};