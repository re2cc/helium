<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    import type { Item } from "../../../types/types";

    // Define the props the component accepts using Svelte 5's Props interface
    type Props = {
        items?: Item[];
    };

    // Destructure props with default values if needed
    const { items }: Props = $props();

    // Helper to format currency (adjust locale and currency as needed)
    function formatCurrency(value: number): string {
        return value.toLocaleString("en-US", {
            style: "currency",
            currency: "USD", // Change this to your desired currency (e.g., 'MXN')
        });
    }

    function selectProduct(item: Item): void {
        invoke("my_custom_command", { item: item });
    }
</script>

<div
    class="border border-gray-300 rounded-lg overflow-hidden flex flex-col bg-white shadow h-full flex-grow min-h-0"
>
    <div
        class="flex border-b border-gray-300 bg-gray-50 font-semibold text-sm text-gray-700"
    >
        <div class="w-3/5 py-2 px-4 text-left">Product</div>
        <div class="w-1/5 py-2 px-4 text-right">Price</div>
        <div class="w-1/5 py-2 px-4 text-right">Quantity</div>
    </div>

    <div class="overflow-y-auto flex-grow min-h-0">
        {#if items === undefined}
            <div class="p-4 text-center text-gray-500 italic">Loading...</div>
        {:else if items.length > 0}
            {#each items as item, i (item.name + i)}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div
                    class="flex border-b border-gray-200 last:border-b-0 hover:bg-gray-50 text-sm text-gray-800"
                    onclick={() => selectProduct(item)}
                >
                    <div
                        class="w-3/5 py-2 px-4 text-left truncate"
                        title={item.name}
                    >
                        {item.name}
                    </div>
                    <div class="w-1/5 py-2 px-4 text-right">
                        {formatCurrency(item.price)}
                    </div>
                    <div class="w-1/5 py-2 px-4 text-right">
                        {item.quantity}
                    </div>
                </div>
            {/each}
        {:else}
            <div class="p-4 text-center text-gray-500 italic">
                No items to display.
            </div>
        {/if}
    </div>
</div>
