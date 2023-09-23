// https://www.youtube.com/watch?v=2DbX0xFL0nk
import React, { useRef, useState } from "react";
import { motion, AnimatePresence, useMotionValue, useSpring } from "framer-motion";
import { Text } from "@components/Text/TextComponent";
import Button from "@mui/material/Button";
import { useTheme } from "@mui/material/styles";
import Box from "@mui/material/Box";
// import { Swipeable } from "react-swipeable";

const slides = [{url: "hello0"}, {url: "hello1"}, {url: "hello2"}, {url: "hello3"}, {url: "hello4"}];
export function Swipe({  }) {
    const [activeSlide, setActiveSlide] = useState(0);
    const itemsRef = useRef<(HTMLDivElement | null)[]>([]);
    const canScrollPrev = activeSlide > 0;
    const canScrollNext = activeSlide < slides.length - 1;
    const offsetX = useMotionValue(0);
    const theme = useTheme();
    const sizeFactor = 0.8;
    const containerSize = 450;
    const itemSize = 300;
    const animatedX = useSpring(offsetX, {
        damping: 20,
        stiffness: 150,
    });
    const onLeft = () => {
        if (activeSlide > 0) {
            setActiveSlide(activeSlide - 1);

        }
    };

    const onRight = () => {
        if (activeSlide < slides.length - 1) {
            setActiveSlide(activeSlide + 1);

        }
    };

    console.log("activeSlide", activeSlide);
    function scrollPrev() {


        //prevent scrolling past first item
        if (!canScrollPrev) return;

        const nextWidth = itemsRef.current
            .at(activeSlide - 1)
            ?.getBoundingClientRect().width ;

        if (nextWidth === undefined) return;
        offsetX.set(offsetX.get() + (nextWidth / sizeFactor));

        setActiveSlide((prev) => prev - 1);
    }
    function scrollNext() {
        // prevent scrolling past last item
        if (!canScrollNext) return;

        const nextWidth = itemsRef.current
            .at(activeSlide + 1)
            ?.getBoundingClientRect().width;
        if (nextWidth === undefined) return;
        offsetX.set(offsetX.get() - (nextWidth / sizeFactor));

        setActiveSlide((prev) => prev + 1);
    }

    function handleClick(e) {
        console.log("e", e);
        // if (e.currentTarget === e.target) {
        const clickedX = e.clientX;
        const currentX = itemsRef.current[activeSlide]?.getBoundingClientRect().x;
        if (currentX === undefined) return;

        // Continue code if <
        if (clickedX < currentX) {
            scrollPrev();
        } else{
            scrollNext();
        }
        e.stopPropagation();
        // }
        // console.log("el", el);
        // console.log("clicked", el.clientX);
        // console.log("current", itemsRef.current[activeSlide]?.getBoundingClientRect().x);
    }



    return (
        <div className="relative overflow-hidden"
            style={{
                // x: animatedX,
                // boxShadow: `inset 0 0 0 1px ${theme.palette.text.primary}`,
                boxShadow: `inset 0 0 0 1px red`,
                maxWidth: containerSize
            }}
        >
            {/*<div className="flex flew-row max-w-[400px] relative overflow-hidden">*/}
            <motion.div
                className="flex flew-row relative"
                style={{
                    x: animatedX,
                    // boxShadow: `inset 0 0 0 1px ${theme.palette.text.primary}`,
                    boxShadow: `inset 0 0 0 1px blue`,
                    left: containerSize/2 - itemSize/2,
                }}
            >
                {slides.map(({ url }, index) => (
                    <motion.div
                        key={index}
                        ref={(el) => (itemsRef.current[index] = el)}
                        // className="overflow-hidden"
                        initial={{ scale: 0 }}
                        animate={{
                            rotate: 0,
                            // x: direction === "right" ? "100%" : "-100%",
                            // x: `${(position) * 100}%`,
                            // x: "100%",
                            scale: index === activeSlide ? 1 : sizeFactor,
                            y: index === activeSlide ? 0 : "-10%"
                        }}
                        style={{
                            // x: animatedX,
                            boxShadow: `inset 0 0 0 1px ${theme.palette.text.primary}`,
                        }}
                        onClick={handleClick}

                        // transition={{
                        //     type: "spring",
                        //     stiffness: 100,
                        //     damping: 10,
                        // }}
                    >
                        <div
                            className="h-full flex"
                            onClick={handleClick}
                            style={{width: itemSize}}
                        >
                            {/*<div*/}
                            {/*    component="div"*/}
                            {/*    position="relative"*/}
                            {/*    height={"100%"}*/}
                            {/*    rowGap={"16px"}*/}
                            {/*    display={"flex"}*/}
                            {/*    alignSelf={"start"}*/}
                            {/*    width={itemSize}*/}
                            {/*>*/}
                            <Text.H1>{url}</Text.H1>
                        </div>
                    </motion.div>
                ))}
            </motion.div>
            <Button
                variant={"contained"}
                onClick={scrollPrev}
            >
                Left
            </Button>
            <Button
                variant={"contained"}
                onClick={scrollNext}
            >
                Right
            </Button>
        </div>
    );
}