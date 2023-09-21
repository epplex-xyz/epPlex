import React, { useEffect, useRef, useState } from "react";
import { Text } from "@components/Text/TextComponent";

export function getTimeRemaining(endTime: Date) {
    const currentTime = new Date();
    // Difference in seconds
    const total = Math.floor((endTime.getTime() - currentTime.getTime()) / 1000);

    const seconds = total % 60;
    const minutes = Math.floor((total / 60) % 60);
    const hours = Math.floor(total / 3600);

    return {total, hours, minutes, seconds};
}

export function startTimer(endTime: Date, setTimer: React.Dispatch<React.SetStateAction<string>>) {
    const { total, hours, minutes, seconds } = getTimeRemaining(endTime);

    if (total >= 0) {
        const formattedHours = hours > 9 ? hours : '0' + hours;
        const formattedMinutes = minutes > 9 ? minutes : '0' + minutes;
        const formattedSeconds = seconds > 9 ? seconds : '0' + seconds;

        setTimer(`${formattedHours}h:${formattedMinutes}m:${formattedSeconds}s`);
    }
}
export function Timer({endTimestamp}: {endTimestamp: number}) {
    const [timer, setTimer] = useState("");
    const ref = useRef<any>(null);

    // Timer is a bit delayed, but the other stuff loads fast enough?
    // That is prolly due to data-fetching speed
    useEffect(() => {
        let intervalId;

        if (ref.current) {
            clearInterval(ref.current);
        }

        const updateTimer = () => {
            startTimer(new Date(endTimestamp * 1000), setTimer);
        };

        intervalId = setInterval(updateTimer, 1000);
        ref.current = intervalId;

        return () => {
            clearInterval(intervalId);
        };
    }, [endTimestamp]);

    return (
        <>
            {timer ? timer : "00:00:00" }
        </>
    );
}