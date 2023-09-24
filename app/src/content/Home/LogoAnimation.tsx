import { motion } from "framer-motion";
import Image from "next/image";
import React from "react";

export function LogoAnimation() {
    const container = {
        hidden: { opacity: 0 },
        show: {
            opacity: 1,
            transition: {
                staggerChildren: 0.5,
                delayChildren: 0.5
            }
        }
    };

    const item = {
        hidden: {
            opacity: 0,
            x: -200
        },
        show: {
            opacity: 1,
            x: 0,
            // animate: {
            //     x: [0, 200],
            // },
            transition: {
                duration: 1,
                ease: [0.02, 0.6, 0.01, 0.91]
            }
        }
    };
    const AR = 125/48;
    const size = 50;

    return (
        <div
            style={{
                width: 400,
                height: 400,
                position: "relative",
                display: "flex",
                alignItems: "center",
                flexDirection: "column",
            }}
        >
            <motion.div
                variants={container}
                initial="hidden"
                animate="show"
            >
                <motion.div
                    style={{
                        // width: 120,
                        // height: 120,
                        // borderRadius: 25,
                        // position: "absolute",
                        // left: 40,
                        // top: 40
                    }}
                    // animate={{
                    //     x: [0, 200],
                    //     // y: [0, 0, 200, 200, 0],
                    //     // backgroundColor: ["#fd3", "#60f", "#fd3"]
                    // }}
                    // transition={{ duration: 1, ease: [0.02, 0.6, -0.01, 0.91] }}
                    variants={item}
                >
                    <Image
                        src={"/logos/animation/e1.png"}
                        alt={"logo"}
                        height={size}
                        width={size * AR}
                    />
                </motion.div>
                <motion.div
                    style={{
                        // position: "absolute",
                        // left: 40,
                        // top: 40
                    }}
                    // animate={{
                    //     x: [0, 200],
                    //     // y: [100, 100, 300, 300, 100],
                    //     // rotate: -360,
                    //     // backgroundColor: ["#fd3", "#60f", "#fd3"]
                    // }}
                    // transition={{ duration: 1, ease: [0.02, 0.6, 0.01, 0.91] }}
                    variants={item}
                >
                    <Image
                        src={"/logos/animation/e2.png"}
                        alt={"logo"}
                        height={size}
                        width={size * AR}
                    />
                </motion.div>
            </motion.div>
        </div>
    );
}
