import { invoke } from "@tauri-apps/api/core";

import type { BasicItem, CurrentItem } from "@/lib/types.ts";
import { currentItem } from "@/lib/stores.ts";

export function selectProduct(barcode: String): void {
    invoke<CurrentItem>("select_item", { barcode: barcode }).then((response) => {
        currentItem.set(response);
    });
}

export function searchProduct(searchValue: string): Promise<BasicItem[]> {
    return invoke<BasicItem[]>("search_product", { searchValue: searchValue }).then((response) => {
        return response;
    });
}
