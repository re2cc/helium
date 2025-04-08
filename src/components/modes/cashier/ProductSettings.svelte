<script>
    import { currentItem } from "@/lib/state.svelte.ts";
    import { Button } from "bits-ui";
</script>

<div
    class="bg-white rounded-xl border-4 border-green-500 p-4 h-full overflow-auto"
>
    <h3>Configuracion</h3>
    {#if currentItem.basic_item.barcode !== ""}
        <div>
            <h3>{currentItem.basic_item.barcode}</h3>
            <Button.Root class="rounded-input bg-dark text-background shadow-mini caret-orange-300">Hola</Button.Root>
        </div>
    {:else}
        <p>This is section 2 with green border.</p>
        <p>Content for section 2.</p>
    {/if}
</div>
