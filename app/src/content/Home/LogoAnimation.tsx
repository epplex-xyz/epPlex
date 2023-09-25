import { motion } from "framer-motion";
import Image from "next/image";
import React from "react";
import Box from "@mui/material/Box";

const container = {
    hidden: { opacity: 0 },
    show: {
        opacity: 1,
        transition: {
            delay: 1,
            staggerChildren: 0.2,
            delayChildren: 1.2
        }
    }
};

const container2 = {
    hidden: { opacity: 0 },
    show: {
        opacity: 1,
        transition: {
            delay: 1.6,
            staggerChildren: 0.2,
            delayChildren: 1.6
        }
    }
};

const itemE = (offset) => {
    return {
        hidden: {
            opacity: 0,
            x: -90 + offset,
        },
        show: {
            opacity: 1,
            x: 90 + offset,
            transition: {
                duration: 1,
                ease: [0.02, 0.6, 0.01, 0.91]
            }
        }
    };
};

const itemP = (offset) => {
    return {
        hidden: {
            opacity: 0,
            x: 120 + offset,
            y: -200
        },
        show: {
            opacity: 1,
            x: 20 + offset,
            y: 0,
            transition: {
                duration: 1,
                ease: [0.02, 0.6, 0.01, 0.91]
            }
        }
    };
};

// Framer makes it hard to animate for mobile and desktop at the same time
export function LogoAnimation() {
    const mobileFactor = 1;

    const AR = 125/48;
    const ARp1 = 136/175;
    const ARp2 = 97/110;
    const sizeE = 50 * mobileFactor;
    const sizeP1 = 165 * mobileFactor;
    const sizeP2 = 112 * mobileFactor;
    const sizes = {
        e: sizeE,
        p1: sizeP1,
        p2: sizeP2
    };

    const offset = {
        p1: 0,
        p2: -40
    };

    const eLogo = <Image
        src={"/logos/animation/e.png"}
        alt={"logo"}
        height={sizes.e}
        width={sizes.e * AR}
    />;
    const p1Logo = <Image
        src={"/logos/animation/p1.png"}
        alt={"logo"}
        height={sizes.p1}
        width={sizes.p1 * ARp1}
    />;
    const p2Logo = <Image
        src={"/logos/animation/p2.png"}
        alt={"logo"}
        height={sizes.p2}
        width={sizes.p2 * ARp2}
    />;

    return (
        <Box
            component={"div"}
            className={"sm:mt-[100px] mt-[50px]"}
            sx={{
                width: "100%",
                position: "relative",
                display: "flex",
                flexDirection: "row",
                justifyContent: "center",
            }}
        >
            {/* E part */}
            <motion.div
                variants={container}
                initial="hidden"
                animate="show"
            >
                <motion.div variants={itemE(0)}>
                    {eLogo}
                </motion.div>
                <motion.div variants={itemE(-55)}>
                    {eLogo}
                </motion.div>
                <motion.div variants={itemE(-65)}>
                    {eLogo}
                </motion.div>
            </motion.div>

            {/* P part */}
            <motion.div
                variants={container2}
                initial="hidden"
                animate="show"
                className="flex flex-row"
            >
                <motion.div variants={itemP(offset.p1)}>
                    {p1Logo}
                </motion.div>
                <motion.div variants={itemP(offset.p2)}>
                    {p2Logo}
                </motion.div>
            </motion.div>
        </Box>
    );
}