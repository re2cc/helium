import { invoke } from "@tauri-apps/api/core";

import type { BasicItem, CurrentItem } from "@/lib/types.ts";
import { currentItem } from "@/lib/state.svelte";

export function selectProduct(barcode: String): void {
    invoke<CurrentItem>("select_item", { barcode: barcode }).then(
        (response) => {
            currentItem.basic_item = response.basic_item;
            currentItem.config_item = response.config_item;
        },
    );
}

export async function searchProduct(searchValue: string): Promise<BasicItem[]> {
    return await invoke<BasicItem[]>("search_product", {
        searchValue: searchValue,
    });
}
