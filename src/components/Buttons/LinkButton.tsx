// Taken from dReader-frontend
import Button, { ButtonProps } from "@mui/material/Button";
import Link from "next/link";
import React from "react";

export interface ButtonLinkProps extends ButtonProps<'a'> {
    Icon?: () => React.ReactNode
    blank?: boolean
}

export const ButtonLink: React.FC<ButtonLinkProps> = ({ Icon, href, blank = false, children, ...props }) => {
    if (!href) return null;

    return (
        <Button
            {...props}
            LinkComponent={Link}
            target={blank ? '_blank' : undefined}
            href={href || '#'}
        >
            {children}

            {Icon && <Icon/>}
        </Button>
    );
};
