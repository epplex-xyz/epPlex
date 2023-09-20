import React from "react";
import { Creation } from "./Creation";
import { MyEpNFTs } from "./MyEpNFTs";

export function DemoPage() {

    return (
        <div className="flex flex-row justify-center gap-x-8">
            <Creation/>
            <MyEpNFTs/>
        </div>
    );
}
