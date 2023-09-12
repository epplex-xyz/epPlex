import { ButtonLinkProps } from "@components/Buttons/LinkButton";
import ArrowForwardIcon from '@mui/icons-material/ArrowForward';
import InsertDriveFileIcon from '@mui/icons-material/InsertDriveFile';

export const ButtonConfig: Record<string, ButtonLinkProps> = {
    app: {
        variant: "contained",
        href: "/app",
        children: <>Launch App</>,
        Icon: () => <ArrowForwardIcon/>,
        sx: {
            color: 'white',
            background: 'linear-gradient(270deg, #3B6EFF 0%, #27D8FF 100%)'
        }
    },
    docs: {
        variant: "outlined",
        href: "/docs",
        children: <>Read Docs</>,
        Icon: () => <InsertDriveFileIcon/>
    },
    access: {
        variant: "contained",
        href: "/access",
        children: <>Request Early Access</>,
        sx: {
            color: '#438EF7',
            background: '#ECF3FE',
            height: "24px"
        }
    }
};