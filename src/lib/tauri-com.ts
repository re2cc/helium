import { invoke } from "@tauri-apps/api/core";

import type { BasicItem, CurrentItem } from "@/lib/types.ts";
import { setCurrentItem } from "@/lib/state.svelte";

export function selectProduct(barcode: String): void {
    invoke<CurrentItem>("select_item", { barcode: barcode }).then(
        (response) => {
            setCurrentItem(response);
        },
    );
}

export async function searchProduct(searchValue: String): Promise<BasicItem[]> {
    return await invoke<BasicItem[]>("search_product", {
        searchValue: searchValue,
    });
}

export async function addProduct(item: CurrentItem): Promise<Boolean> {
    return await invoke<Boolean>("add_product", {
        barcode: item.basic_item.barcode,
        itemConfig: item.config_item,
    });
}
