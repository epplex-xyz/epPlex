// Taken from dReader-frontend
import Button, { ButtonProps } from "@mui/material/Button";
import Link from "next/link";
import React from "react";

type LinkType = "internal" | "external";

export interface ButtonLinkProps extends ButtonProps<'a'> {
    Icon?: () => React.ReactNode
    blank?: boolean
    linkType?: LinkType
}

export const ButtonLink: React.FC<ButtonLinkProps> = ({ Icon, href, blank = false, children, linkType = "internal", ...props }) => {
    if (!href) return null;

    return (
        <Button
            {...props}
            LinkComponent={
                linkType === "internal" ? Link : undefined
            }
            target={blank ? '_blank' : undefined}
            href={href}
        >
            {children}

            {Icon && <Icon/>}
        </Button>
    );
};
