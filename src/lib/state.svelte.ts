import { atom } from "nanostores";

import type { CurrentItem } from "@/lib/types.ts";

export const currentItem = $state<CurrentItem>({
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
