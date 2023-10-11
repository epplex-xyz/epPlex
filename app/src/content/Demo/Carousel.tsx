// https://www.youtube.com/watch?v=2DbX0xFL0nk
import React, { useRef, useState } from "react";
import { motion, useMotionValue, useSpring } from "framer-motion";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import { BWIconButton } from "@components/Buttons/IconButton";

interface CarouselProps<T> {
    items: T[];
    ItemComponent: (props: {item: T}) => React.ReactElement
}

export function Carousel<T>({items, ItemComponent}: CarouselProps<T>) {
    const [activeSlide, setActiveSlide] = useState(0);
    const itemsRef = useRef<(HTMLDivElement | null)[]>([]);
    const canScrollPrev = activeSlide > 0;
    const canScrollNext = activeSlide < items.length - 1;
    const offsetX = useMotionValue(0);
    const sizeFactor = 0.8;

    // a bit hardcoded, but fuk it
    const containerSize = 400;
    const itemSize = 300;

    const animatedX = useSpring(offsetX, {
        damping: 20,
        stiffness: 150,
    });

    function onPrev() {
        //prevent scrolling past first item
        if (!canScrollPrev) return;

        const nextWidth = itemsRef.current
            .at(activeSlide - 1)
            ?.getBoundingClientRect().width ;

        if (nextWidth === undefined) return;
        offsetX.set(offsetX.get() + (nextWidth / sizeFactor));

        setActiveSlide((prev) => prev - 1);
    }

    function onNext() {
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
        const clickedX = e.clientX;
        const currentX = itemsRef.current[activeSlide]?.getBoundingClientRect().x;
        if (currentX === undefined) return;

        if (clickedX < currentX) {
            onPrev();
        } else{
            onNext();
        }

        e.stopPropagation();
    }

    return (
        <>
            <div
                className="relative overflow-hidden"
                style={{
                    maxWidth: containerSize,
                    paddingBottom: "40px"
                }}
            >
                <div className="flex justify-center gap-x-8 pb-4">
                    <BWIconButton onClick={onPrev} disabled={!canScrollPrev}>
                        <ChevronLeftIcon/>
                    </BWIconButton>
                    <BWIconButton onClick={onNext} disabled={!canScrollNext}>
                        <ChevronRightIcon/>
                    </BWIconButton>
                </div>

                <motion.div
                    className="flex flew-row relative"
                    style={{
                        x: animatedX,
                        left: containerSize/2 - itemSize/2,
                    }}
                >
                    {items.map((item, index) => (
                        <motion.div
                            key={index}
                            ref={(el) => (itemsRef.current[index] = el)}
                            initial={{ scale: 0 }}
                            animate={{
                                rotate: 0,
                                scale: index === activeSlide ? 1 : sizeFactor,
                                y: index === activeSlide ? 0 : "-5%"
                            }}
                            onClick={handleClick}
                        >
                            <ItemComponent item={item}/>
                        </motion.div>
                    ))}
                </motion.div>

                {/*<div className={"absolute bot-0 left-0"}>*/}
                {/*    <BWIconButton onClick={onPrev} disabled={!canScrollPrev}>*/}
                {/*        <ChevronLeftIcon/>*/}
                {/*    </BWIconButton>*/}
                {/*</div>*/}
                {/*<div className={"absolute bot-0 right-0"}>*/}
                {/*    <BWIconButton onClick={onNext} disabled={!canScrollNext}>*/}
                {/*        <ChevronRightIcon/>*/}
                {/*    </BWIconButton>*/}
                {/*</div>*/}
                {/* Buttons */}
                {/*<div className="flex justify-center gap-x-4 py-4">*/}


                {/*</div>*/}
            </div>

        </>
    );
}