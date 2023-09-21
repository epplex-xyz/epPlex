import React from "react";
import { Creation } from "./Creation";
import { MyEpNFTs } from "./MyEpNFTs";

export function DemoPage() {
    return (
        // <div className="flex flex-row flex-wrap justify-center gap-x-8 h-full">
        <div className="flex flex-wrap justify-center items-center gap-x-8">
            <Creation/>
            <MyEpNFTs/>
        </div>
    );
}
