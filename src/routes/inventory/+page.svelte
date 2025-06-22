<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import ProductList from '$lib/components/pos/productlist.svelte';
  import AddItemEditItemSection, { type Item } from '$lib/components/inventory/AddItemEditItemSection.svelte';

  let inventoryItems: Item[] = [];
  let selectedItemForEdit: Item | null = null;
  let errorMessage = "";
  let loading = true;

  async function loadInventory() {
    loading = true;
    errorMessage = "";
    try {
      inventoryItems = await invoke<Item[]>("get_inventory");
    } catch (err) {
      errorMessage = "Failed to load inventory.";
    }
    loading = false;
  }

  async function handleAddItem(event: CustomEvent<{ name: string; price: number; stock: number }>) {
    try {
      const newItem = await invoke<Item>("add_product", event.detail);
      inventoryItems = [...inventoryItems, newItem];
      selectedItemForEdit = null;
    } catch (err) {
      errorMessage = "Failed to add item.";
    }
  }

  async function handleUpdateItem(event: CustomEvent<Item>) {
    try {
      const updatedItem = await invoke<Item>("update_product", event.detail);
      inventoryItems = inventoryItems.map(item =>
        item.id === updatedItem.id ? updatedItem : item
      );
      selectedItemForEdit = null;
    } catch (err) {
      errorMessage = "Failed to update item.";
    }
  }

  async function handleDeleteItem(event: CustomEvent<string>) {
    try {
      await invoke("delete_product", { id: event.detail });
      inventoryItems = inventoryItems.filter(item => item.id !== event.detail);
      selectedItemForEdit = null;
    } catch (err) {
      errorMessage = "Failed to delete item.";
    }
  }

  function handleClearSelection() {
    selectedItemForEdit = null;
  }

  function handleProductSelect(event: CustomEvent<Item>) {
    selectedItemForEdit = event.detail;
  }

  onMount(loadInventory);
</script>

<div class="inventory-page-layout">
  {#if loading}
    <p>Loading inventory...</p>
  {:else}
    {#if errorMessage}
      <p style="color: red;">{errorMessage}</p>
    {/if}
    <ProductList
      products={inventoryItems}
      on:productSelected={handleProductSelect}
      showQuantityControls={false}
    />

    <AddItemEditItemSection
      bind:selectedItem={selectedItemForEdit}
      on:addItem={handleAddItem}
      on:updateItem={handleUpdateItem}
      on:deleteItem={handleDeleteItem}
      on:clearSelection={handleClearSelection}
    />
  {/if}
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