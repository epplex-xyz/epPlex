import React from "react";
import Particles from "react-particles";
import type { Engine } from "tsparticles-engine";
import { loadSnowPreset } from "tsparticles-preset-snow";

export function Snow() {
    const options = {
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
    };

    const init = async (engine: Engine) =>  await loadSnowPreset(engine);

    return (
        <Particles
            options={options}
            init={init}
        />
    );
}
