import { motion } from "framer-motion";
import Image from "next/image";
import React from "react";
import Box from "@mui/material/Box";
import useMediaQuery from "@mui/material/useMediaQuery";
import { useTheme } from "@mui/material/styles";

export function LogoAnimation() {
    const isMobile = useMediaQuery(useTheme().breakpoints.down('sm'));
    const mobileFactor = isMobile ? 0.8 : 1;
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

    const item = (offset) => {
        return {
            hidden: {
                opacity: 0,
                x: -200 + offset,
            },
            show: {
                opacity: 1,
                x: 0 + offset,
                // animate: {
                //     x: [0, 200],
                // },
                transition: {
                    duration: 1,
                    ease: [0.02, 0.6, 0.01, 0.91]
                }
            }
        };
    };

    // const itemY = (offset) => {
    //     return {
    //         hidden: {
    //             opacity: 0,
    //             x: 160 + offset,
    //             y: -370 ,
    //         },
    //         show: {
    //             opacity: 1,
    //             x: 60 + offset,
    //             y: -170,
    //             transition: {
    //                 duration: 1,
    //                 ease: [0.02, 0.6, 0.01, 0.91]
    //             }
    //         }
    //     };
    // };
    //
    // const itemY2 = (offset) => {
    //     return {
    //         hidden: {
    //             opacity: 0,
    //             x: 160 + offset,
    //             y: -544,
    //         },
    //         show: {
    //             opacity: 1,
    //             x: 60 + offset,
    //             y: -344,
    //             transition: {
    //                 duration: 1,
    //                 ease: [0.02, 0.6, 0.01, 0.91]
    //             }
    //         }
    //     };
    // };

    const itemY = (offset) => {
        return {
            hidden: {
                opacity: 0,
                x: 160 + offset,
                y: -370 ,
            },
            show: {
                opacity: 1,
                x: 60 + offset,
                y: -170,
                transition: {
                    duration: 1,
                    ease: [0.02, 0.6, 0.01, 0.91]
                }
            }
        };
    };

    const itemY2 = (offset) => {
        return {
            hidden: {
                opacity: 0,
                x: 30 + offset,
                // y: -200
            },
            show: {
                opacity: 1,
                x: -70 + offset,
                // y: 0,
                transition: {
                    duration: 1,
                    ease: [0.02, 0.6, 0.01, 0.91]
                }
            }
        };
    };


    const AR = 125/48;
    const sizeE = 50 * mobileFactor;
    const eLogo = <Image
        src={"/logos/animation/e.png"}
        alt={"logo"}
        height={sizeE}
        width={sizeE * AR}
    />;

    const ARp1 = 136/175;
    const sizeP1 = 168 * mobileFactor;
    const p1Logo = <Image
        src={"/logos/animation/p1.png"}
        alt={"logo"}
        height={sizeP1}
        width={sizeP1 * ARp1}
    />;

    const ARp2 = 97/110;
    const sizeP2 = 112 * mobileFactor;
    const p2Logo = <Image
        src={"/logos/animation/p2.png"}
        alt={"logo"}
        height={sizeP2}
        width={sizeP2 * ARp2}
    />;

    const offset1 = isMobile ? 80 : 0;
    const offset2 = isMobile ? 40 : -40;

    console.log("offser", offset1, offset2);
    return (
        <Box
            component={"div"}
            className={"sm:mt-[100px] mt-[50px]"}
            // marginTop={{sm: "50px", md: "100px"}}
            sx={{
                // left: 75,
                width: "100%",
                position: "relative",
                display: "flex",
                flexDirection: "row",
                justifyContent: "center",
            }}
        >
            {/* E part */}
            {/*<div className="absolute ">*/}
            {/*    <div className={"flex flex-row"}>*/}
            <motion.div
                variants={container}
                initial="hidden"
                animate="show"
            >
                <motion.div variants={item(0)}>
                    {eLogo}
                </motion.div>
                <motion.div variants={item(-55)}>
                    {eLogo}
                </motion.div>
                <motion.div variants={item(-65)}>
                    {eLogo}
                </motion.div>
            </motion.div>

            {/* P part */}
            <motion.div
                variants={container}
                initial="hidden"
                animate="show"
                className="flex flex-row"
            >
                <motion.div variants={itemY2(offset1)}>
                    {p1Logo}
                </motion.div>
                <motion.div variants={itemY2(offset2)}>
                    {p2Logo}
                </motion.div>
            </motion.div>
            {/*    </div>*/}
            {/*</div>*/}
        </Box>
    );
}

