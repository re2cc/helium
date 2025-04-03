<script lang="ts">
    import List from "./List.svelte";

    const productItems = [
        { name: "Laptop", price: 1200, quantity: 5 },
        { name: "Mouse", price: 25, quantity: 20 },
        { name: "Keyboard", price: 75, quantity: 10 },
        { name: "Monitor", price: 300, quantity: 8 },
        { name: "Laptop", price: 1200, quantity: 5 },
        { name: "Mouse", price: 25, quantity: 20 },
        { name: "Keyboard", price: 75, quantity: 10 },
        { name: "Monitor", price: 300, quantity: 8 },
        { name: "Laptop", price: 1200, quantity: 5 },
        { name: "Mouse", price: 25, quantity: 20 },
        { name: "Keyboard", price: 75, quantity: 10 },
        { name: "Monitor", price: 300, quantity: 8 },
        { name: "Laptop", price: 1200, quantity: 5 },
    ];
</script>

<div
    class="bg-white rounded-xl border-4 border-orange-500 p-4 h-full flex flex-col"
>
    <input
        type="text"
        placeholder="Search for products..."
        class="w-full p-2 border border-gray-300 rounded mb-4"
    />

    <!-- Simplified but more explicit structure -->
    <List items={productItems} />
</div>
