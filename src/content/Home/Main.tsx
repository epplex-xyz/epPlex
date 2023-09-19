import React from "react";
import { LandingPage } from "./LandingPage";
import { SectionConfig } from "./sections";
import Particles from "react-particles";
import type { Engine } from "tsparticles-engine";
import { loadSnowPreset } from "tsparticles-preset-snow";

export function Main() {
    return (
        <div className="flex flex-col w-full">
            <Particles
                options={{
                    background: {
                        color: {
                            value: "transparent"
                        }
                    },
                    particles: {
                        number: {
                            value: 50,
                        }
                    },
                    preset: "snow",
                }}
                init={async (engine: Engine) =>  await loadSnowPreset(engine)}
            />
            <LandingPage id={SectionConfig.landingPage.id}/>
        </div>
    );
}
