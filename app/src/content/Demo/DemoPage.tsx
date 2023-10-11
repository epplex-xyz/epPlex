import React from "react";
import { Creation } from "./Creation";
import { MyEpNFTs } from "./MyEpNFTs";

export function DemoPage() {
    return (
        <div className="flex flex-row flex-wrap justify-center items-center gap-x-48">
            <Creation/>
            <MyEpNFTs/>
        </div>
    );
}
