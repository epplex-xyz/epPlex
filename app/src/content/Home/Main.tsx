import React from "react";
import { LandingPage } from "./LandingPage";
import { SectionConfig } from "./sections";
import { Snow } from "@components/Animation/Snow";

export function Main() {
    return (
        <div className="flex flex-col w-full">
            <Snow/>
            <LandingPage id={SectionConfig.landingPage.id}/>
        </div>
    );
}
