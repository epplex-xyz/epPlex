import { ButtonLinkProps } from "@components/Buttons/LinkButton";

export const ButtonConfig: Record<string, ButtonLinkProps> = {
    demo: {
        variant: "contained",
        href: "/demo",
        children: <>Try Demo</>,
        sx: {
            paddingX: '32px',
        }
    },
    docs: {
        variant: "outlined",
        href: "/docs",
        children: <>Stay Updated</>,
        sx: {
            paddingX: '32px',
        }
    },
};