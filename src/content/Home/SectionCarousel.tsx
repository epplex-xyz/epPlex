import React, { useRef } from "react";
import { motion, MotionValue, useScroll, useTransform } from "framer-motion";
import { Section } from "@components/Container/Section";
import { Text } from "@components/Text/TextComponent";

// athos.framer.website continues to move back even after the subsequent target has passed

function useCardScroll(endPercentage): { containerRef: React.RefObject<HTMLDivElement>, scaleValue: MotionValue} {
    const containerRef = useRef<HTMLDivElement | null>(null);
    const scroll = useScroll({
        target: containerRef,
        // Start monitoring, when the start of the target meets the end of the container
        // container is per default the viewport
        // End monitoring when the end of the target meets the end of the container
        offset: ["start end", "end end"]
    });
    const scaleValue = useTransform(scroll.scrollYProgress, [0, 1], ["100%", endPercentage]);

    return { containerRef, scaleValue};
}


export function SectionCarousel({...props}){
    const card1 = useCardScroll("85%");
    const card2 = useCardScroll("90%");
    const card3 = useCardScroll("95%");
    const card4 = useCardScroll("100%");

    const spring = {
        type: "spring",
        damping: 69,
        mass: 2.3,
        stiffness: 422
    };

    // how to make card1 card2 card3 continue to scale down as you scroll
    const cards = [
        {ref: card1.containerRef, scaleValue: card2.scaleValue, color: "green"},
        {ref: card2.containerRef, scaleValue: card3.scaleValue, color: "orange"},
        {ref: card3.containerRef, scaleValue: card4.scaleValue, color: "purple"},
        {ref: card4.containerRef, scaleValue: "1", color: "black"},
    ];

    return (
        <Section
            {...props}
            height={"min-content"}
            flexDirection={"column"}
            overflow={"visible"}
            rowGap={"50px"}
        >
            {cards.map(({ref, scaleValue, color}, index) =>
                <motion.div
                    key={`Card${index}`}
                    ref={ref}
                    transition={spring}
                    style={{
                        scale: scaleValue,
                        height: "80vh",
                        position: "sticky",
                        display: "flex",
                        justifyContent: "center",
                        alignItems: "center",
                        width: "100%",
                        top: `${100 + index * 100}px`,
                        backgroundColor: color,
                    }}
                >
                    <Text.H1>
                        {index}
                    </Text.H1>
                </motion.div>
            )}
        </Section>
    );
}