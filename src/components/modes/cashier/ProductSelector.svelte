<script lang="ts">
    import { searchProduct } from "@/lib/tauri-com.ts";
    import List from "@/components/modes/cashier/List.svelte";

    let searchInput = $state("");
    let result = $derived(searchProduct(searchInput));
</script>

<div
    class="bg-white rounded-xl border-4 border-orange-500 p-4 h-full flex flex-col"
>
    <input
        type="text"
        placeholder="Search for products..."
        class="w-full p-2 border border-gray-300 rounded mb-4"
        bind:value={searchInput}
    />

    {#await result}
        <List />
    {:then items}
        <List {items} />
    {/await}
</div>
