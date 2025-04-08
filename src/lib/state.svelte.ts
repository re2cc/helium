import type { CurrentItem } from "@/lib/types.ts";

let currentItem = $state<CurrentItem | null>(null)

export function setCurrentItem(item: CurrentItem | null) {
    currentItem = item;
}

export function getCurrentItem() {
    return currentItem;
}
