import { Price } from "./price";

interface Product {
    id: string;
    name: string;

    active: boolean;

    description: string;

    images: string[];

    maxSupply?: number;

    tags: string[];

    prices: Price[];

    meta: object;
}

export interface ReturnProduct extends Product {
    id: string;
}
