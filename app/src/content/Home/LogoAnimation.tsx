import { motion } from "framer-motion";
import Image from "next/image";
import React from "react";
import Box from "@mui/material/Box";

export function LogoAnimation() {
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
                y: -200
            },
            show: {
                opacity: 1,
                x: -70 + offset,
                y: 0,
                transition: {
                    duration: 1,
                    ease: [0.02, 0.6, 0.01, 0.91]
                }
            }
        };
    };


    const AR = 125/48;
    const size = 50;

    const eLogo = <Image
        src={"/logos/animation/e.png"}
        alt={"logo"}
        height={size}
        width={size * AR}
    />;

    const ARp1 = 136/175;
    const p1Logo = <Image
        src={"/logos/animation/p1.png"}
        alt={"logo"}
        height={168}
        width={168 * ARp1}
    />;

    const ARp2 = 97/110;
    const p2Logo = <Image
        src={"/logos/animation/p2.png"}
        alt={"logo"}
        height={112}
        width={112 * ARp2}
    />;

    return (
        <Box
            component={"div"}
            sx={{
                left: 75,
                marginTop: {sm: "50px", md: "100px"},
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
            {/*<div*/}
            {/*    style={{*/}
            {/*        width: "100%",*/}
            {/*        // position: "relative",*/}
            {/*        display: "flex",*/}
            {/*        flexDirection: "row",*/}
            {/*        // alignItems: "center",*/}
            {/*    }}*/}
            {/*>*/}
            <motion.div
                variants={container}
                initial="hidden"
                animate="show"
                className="flex flex-row"
            >
                <motion.div variants={itemY2(0)}>
                    {p1Logo}
                </motion.div>
                <motion.div variants={itemY2(-40)}>
                    {p2Logo}
                </motion.div>
            </motion.div>
            {/*</div>*/}
        </Box>
    );
}

