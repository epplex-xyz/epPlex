import React from "react";
import { withStyles } from '@mui/styles';

const cssVariables = (theme) => ({
    '@global': {
        ':root': {
            '--color-primary': theme.palette.primary.main,
        }
    }
});

const CssProvider = ({children}) => {
    return <div>{children}</div>;
};

export default withStyles(cssVariables)(CssProvider);

