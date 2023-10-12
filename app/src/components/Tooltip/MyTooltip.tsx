import * as React from 'react';
import { styled } from '@mui/material/styles';
import Tooltip, { tooltipClasses, TooltipProps } from '@mui/material/Tooltip';
import {useState} from "react";
import IconButton from "@mui/material/IconButton";
import ContentCopyIcon from "@mui/icons-material/ContentCopy";

//https://mui.com/material-ui/react-tooltip/
//https://stackoverflow.com/questions/36759985/how-to-style-mui-tooltip
export const CustomTooltip = styled((props: TooltipProps) => (
    <Tooltip classes={{ popper: props.className }}  {...props} >
        {props.children}
    </Tooltip>
))(({ theme }) => ({
    [`& .${tooltipClasses.tooltip}`]: {
        backgroundColor: theme.palette.secondary.main,
        color: theme.palette.primary.main,
        boxShadow: theme.shadows[1],
        fontSize: 11,
        marginTop: "0px !important",
        transform: "translateY(10%) !important",
        maxWidth: 150,
        textAlign: "center",
    },
    [`& .${tooltipClasses.arrow}`]: {
        color: theme.palette.primary.light,
    },
}));

export function CopyTooltip({copyText}: {copyText: string}) {
    const [copied, setCopied] = useState(false);

    return (
        <CustomTooltip
            arrow
            title={copied ? "Copied" : "Click to copy"}
            placement="top"
        >
            <IconButton
                onClick={(e) => {
                    navigator.clipboard.writeText(copyText);
                    setCopied(true);
                    e.stopPropagation();
                }}
                onMouseLeave={() => setCopied(false)}
                sx={{marginLeft: "8px"}}
            >
                <ContentCopyIcon
                    sx={{
                        height: "14px",
                        width: "14px",
                        color: (theme) => theme.palette.text.primary
                    }}
                />
            </IconButton>
        </CustomTooltip>
    );

}