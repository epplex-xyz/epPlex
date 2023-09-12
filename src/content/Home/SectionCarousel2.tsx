import React, { useRef } from "react";
import { motion, useScroll, useTransform } from "framer-motion";
import { Text } from "@components/Text/TextComponent";

// athos.framer.website continues to move back even after the subsequent target has passed
// Continue to scale down, similar to
// https://nitro.framer.website/
export function SectionCarousel2({...props}){
    const containerRef = useRef<HTMLDivElement | null>(null);
    const scroll = useScroll({
        target: containerRef,
        offset: ["start end", "end end"]
    });

    const divRef = useRef<HTMLDivElement | null>(null);
    const [divHeight, setDivHeight] = React.useState(0);
    const [containerHeight, setContainerHeight] = React.useState(0);
    React.useEffect(() => {
        if (containerRef.current) {
            const { height } = containerRef.current.getBoundingClientRect();
            setContainerHeight(height);
        }

        if (divRef.current) {
            const { height } = divRef.current.getBoundingClientRect();
            setDivHeight(height);
        }

    }, []);
    const translate1 = useTransform(scroll.scrollYProgress, [divHeight/containerHeight, 1], ["0px", "-200px"]);
    const translate2 = useTransform(scroll.scrollYProgress, [divHeight/containerHeight * 2, 1], ["0px", "-200px"]);

    const scaleValue1 = useTransform(scroll.scrollYProgress, [divHeight/containerHeight, 1], ["100%", "90%"]);
    const scaleValue2 = useTransform(scroll.scrollYProgress, [divHeight/containerHeight * 2, 1], ["100%", "95%"]);
    const cards = [
        {scale: scaleValue1, translate: translate1},
        {scale: scaleValue2, translate: translate2},
        {scale: 1, translate: "0px"},
    ];

    return (
        <section
            {...props}
            ref={containerRef}
            style={{
                height: "min-content",
                flexDirection: "column",
                overflow: "visible",
                rowGap: "50px",
                display: "flex",
                marginTop: "100px",
            }}
        >
            {cards.map(({scale, translate}, index) =>
                <motion.div
                    key={`Card${index}`}
                    ref={divRef}
                    style={{
                        scale: scale,
                        translateY: translate,
                        height: "80vh",
                        position: "sticky",
                        display: "flex",
                        justifyContent: "center",
                        alignItems: "center",
                        width: "100%",
                        top: `${100 + index * 100}px`,
                        backgroundColor: "green",
                    }}
                >
                    <Text.H1>
                        {index}
                    </Text.H1>
                </motion.div>
            )}
        </section>
    );
}