<script>
    import { currentItem } from "../../../types/stores.js";
</script>

<div
    class="bg-white rounded-xl border-4 border-green-500 p-4 h-full overflow-auto"
>
    <h3>Section 2</h3>
    <h3>{$currentItem.basic_item.barcode}</h3>
    <p>This is section 2 with green border.</p>
    <p>Content for section 2.</p>
</div>
