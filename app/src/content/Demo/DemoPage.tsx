import React from "react";
import { Creation } from "./Creation";
import { MyEpNFTs } from "./MyEpNFTs";
import { Swipe } from "./Swipe";
import SuggestedCarousel from "./SuggestedCarousel";

export function DemoPage() {
    return (
        <div className="flex flex-row flex-wrap justify-center items-center gap-x-48 h-[1000px]">
            {/*<Creation/>*/}
            {/*<MyEpNFTs/>*/}
            <Swipe/>

            {/*<SuggestedCarousel/>*/}
        </div>
    );
}
