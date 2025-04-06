import { atom } from "nanostores";

import type { CurrentItem } from "./types.ts";

export const currentItem = atom<CurrentItem>({
    basic_item: {
        barcode: "",
        name: "",
        price: 0,
        available_quantity: 0,
    },
    config_item: {
        sell_quantity: 0,
    },
})
