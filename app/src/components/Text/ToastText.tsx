import { explorerURL } from "../../../utils/solana";
import { spliceAddress } from "../../../utils/general";
import React from "react";

export function ToastText({text, signature}: {text: string, signature: string}) {
    return (
        <div className={"flex flex-col"}>
            {text}
            <a
                href={ explorerURL({txSignature: signature })}
                target={"_blank"}
                style={{textDecoration: "underline"}}
            >
                {spliceAddress(signature)}
            </a>
        </div>
    );
}