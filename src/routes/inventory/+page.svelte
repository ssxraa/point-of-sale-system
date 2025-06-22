<script lang="ts">
  import ProductList from '$lib/components/pos/productlist.svelte';
  import AddItemEditItemSection, { type Item } from '$lib/components/inventory//AddItemEditItemSection.svelte';

  // ✨ Mock Inventory Data ✨
  // In a real app, this would come from your backend/database!
  // We'll manage it here for now to demonstrate functionality.
  let inventoryItems = $state<Item[]>([
    { id: 'prod-001', name: 'Sparkling Water (Large)', price: 3.50, stock: 150 },
    { id: 'prod-002', name: 'Organic Coffee Beans (250g)', price: 12.99, stock: 80 },
    { id: 'prod-003', name: 'Artisanal Bread (Whole Wheat)', price: 4.25, stock: 30 },
    { id: 'prod-004', name: 'Vegan Chocolate Bar', price: 5.00, stock: 120 },
    { id: 'prod-005', name: 'Freshly Squeezed Orange Juice (1L)', price: 6.75, stock: 60 },
  ]);

  // State to hold the currently selected item for editing
  let selectedItemForEdit: Item | null = $state(null);

  // --- Event Handlers from ProductList ---
  function handleProductSelect(event: CustomEvent<Item>) {
    selectedItemForEdit = event.detail; // Set the selected item for the edit section
    console.log('Selected item for edit:', selectedItemForEdit);
  }

  // --- Event Handlers from AddItemEditItemSection ---
  function handleAddItem(event: CustomEvent<{ name: string; price: number; stock: number }>) {
    const newItem = {
      id: `prod-${Date.now()}-${Math.floor(Math.random() * 1000)}`, // Simple unique ID for mock data
      ...event.detail
    };
    inventoryItems = [...inventoryItems, newItem]; // Add new item to the list
    console.log('New item added:', newItem);
    selectedItemForEdit = null; // Clear selection after adding
  }

  function handleUpdateItem(event: CustomEvent<Item>) {
    const updatedItem = event.detail;
    inventoryItems = inventoryItems.map(item =>
      item.id === updatedItem.id ? updatedItem : item
    );
    console.log('Item updated:', updatedItem);
    selectedItemForEdit = null; // Clear selection after updating
  }

  function handleDeleteItem(event: CustomEvent<string>) {
    const itemIdToDelete = event.detail;
    inventoryItems = inventoryItems.filter(item => item.id !== itemIdToDelete);
    console.log('Item deleted:', itemIdToDelete);
    selectedItemForEdit = null; // Clear selection after deleting
  }

  function handleClearSelection() {
    selectedItemForEdit = null; // Clears the selection when AddItemEditItemSection requests it
    console.log('Selection cleared.');
  }
</script>

<div class="inventory-page-layout">
    <ProductList
      products={inventoryItems}
      on:productSelected={handleProductSelect}
      showQuantityControls={false} />

    <AddItemEditItemSection
      bind:selectedItem={selectedItemForEdit}
      on:addItem={handleAddItem}
      on:updateItem={handleUpdateItem}
      on:deleteItem={handleDeleteItem}
      on:clearSelection={handleClearSelection}
    />
</div>

<style>
  .inventory-page-layout {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 1.5rem;
    padding: 0;
    background-color: none;
    border-radius: 40px;
    overflow: hidden;
  }

</style>