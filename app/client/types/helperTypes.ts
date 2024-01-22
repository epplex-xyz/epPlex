


export type FixedLengthArray<T, L extends number> = L extends L
    ? number[] extends ((
            ...args: [...Array<L>]
        ) => void)
        ? T[]
        : [...Array<L>]
    : never;