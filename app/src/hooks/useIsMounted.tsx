// https://www.codemzy.com/blog/ismounted-hook-with-useeffect-reactjs
import {useCallback, useEffect, useRef, useState} from 'react';
import {useWallet} from "@solana/wallet-adapter-react";


export function useIsMounted() {
    const [mounted, setMounted] = useState(false);

    useEffect(() => {
        setMounted(true);
    }, []);

    return mounted;
}

export function useMountedWallet() {
    const { connected} = useWallet();
    const [mounted, setMounted] = useState(false);

    useEffect(() => {
        setMounted(true);
    }, [connected]);

    return {mounted, connected};
}