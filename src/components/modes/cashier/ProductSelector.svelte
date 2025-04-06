<script lang="ts">
    import { searchProduct } from "../../../lib/tauri-com.ts";
    import List from "./List.svelte";

    let search_input = $state("");
    let result = $derived.by(() => {
        return searchProduct(search_input);
    });
</script>

<div
    class="bg-white rounded-xl border-4 border-orange-500 p-4 h-full flex flex-col"
>
    <input
        type="text"
        placeholder="Search for products..."
        class="w-full p-2 border border-gray-300 rounded mb-4"
        bind:value={search_input}
    />

    {#await result}
        <List />
    {:then items}
        <List {items} />
    {/await}
</div>
