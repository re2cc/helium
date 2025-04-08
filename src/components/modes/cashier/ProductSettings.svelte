<script lang="ts">
    import type { ConfigItem } from "@/lib/types";

    import { getCurrentItem } from "@/lib/state.svelte";
    import { addProduct } from "@/lib/tauri-com";

    import { Button } from "bits-ui";

    let currentItem = $derived(getCurrentItem());
    let tempItem = $state<ConfigItem>({
        sell_quantity: 1,
    });
</script>

<div
    class="bg-white rounded-xl border-4 border-green-500 p-4 h-full overflow-auto"
>
    <h3>Configuracion</h3>
    {#if currentItem !== null}
        <div>
            <p>Codigo de barras: {currentItem.basic_item.name}</p>
            <p>Codigo de barras: {currentItem.basic_item.barcode}</p>
            <Button.Root
                class="rounded-input bg-dark text-background shadow-mini caret-orange-300"
                onclick={() => (tempItem.sell_quantity -= 1)}
            >
                -
            </Button.Root>
            <p>{tempItem.sell_quantity}</p>
            <Button.Root
                class="rounded-input bg-dark text-background shadow-mini caret-orange-300"
                onclick={() => (tempItem.sell_quantity += 1)}
            >
                +
            </Button.Root>
            <Button.Root
                class="rounded-input bg-dark text-background shadow-mini caret-orange-300"
                onclick={() =>
                    addProduct({
                        basic_item: currentItem.basic_item,
                        config_item: tempItem,
                    })}
            >
                Agregar
            </Button.Root>
        </div>
    {:else}
        <p>Selecciona un producto</p>
    {/if}
</div>
