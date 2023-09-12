import React from "react";
import { LandingPage } from "./LandingPage";
import { SectionCarousel } from "./SectionCarousel";
import { SectionConfig } from "./sections";
import { Investors } from "./Investors";
import { SectionCarousel2 } from "./SectionCarousel2";

export function Main() {
    return (
        <div className="flex flex-col w-full w-full">
            <LandingPage id={SectionConfig.landingPage.id}/>

            <SectionCarousel id={SectionConfig.carousel.id}/>

            <SectionCarousel2 id={SectionConfig.carousel.id}/>

            <Investors id={SectionConfig.investors.id}/>
        </div>
    );
}
