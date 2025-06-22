<script lang="ts">
  import AddItemModal from './AddItemModal.svelte';
  export let selectedItem: Item | null = null;

  let isEditing: boolean = false;
  let showAddModal: boolean = false;

  let currentItemId: string | null = null;
  let currentItemName: string = '';
  let currentItemPrice: number | null = null;
  let currentItemStock: number | null = null;

  $: if (selectedItem) {
    isEditing = true;
    currentItemId = selectedItem.id;
    currentItemName = selectedItem.name;
    currentItemPrice = selectedItem.price;
    currentItemStock = selectedItem.stock;
  } else {
    isEditing = false;
    resetForm();
  }

  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  function resetForm() {
    currentItemId = null;
    currentItemName = '';
    currentItemPrice = null;
    currentItemStock = null;
    isEditing = false;
    dispatch('clearSelection');
  }

  function handleOpenAddModal() {
    showAddModal = true;
  }

  function handleConfirmAdd(event: CustomEvent) {
    const newItemData = event.detail;
    dispatch('addItem', newItemData);
    showAddModal = false;
    resetForm();
  }

  function handleCancelAdd() {
    showAddModal = false;
  }

  function handleSaveItem() {
    if (!currentItemId || !currentItemName.trim() || currentItemPrice === null || currentItemPrice < 0 || currentItemStock === null || currentItemStock < 0) {
      alert('Darling, all fields must be filled correctly for saving!');
      return;
    }
    dispatch('updateItem', {
      id: currentItemId,
      name: currentItemName.trim(),
      price: currentItemPrice,
      stock: currentItemStock
    });
    resetForm();
  }

  function handleDeleteItem() {
    if (!currentItemId) {
      alert('Cannot delete an item that is not selected, bestie!');
      return;
    }
    if (confirm('Are you sure you want to delete this fabulous item? This cannot be undone!')) {
      dispatch('deleteItem', currentItemId);
      resetForm();
    }
  }

  export type Item = {
    id: string;
    name: string;
    price: number;
    stock: number;
  };
</script>

<div class="add-edit-section glassy-background">
  <h2 class="section-title">{isEditing ? 'Edit Item Details' : 'Add New Item'}</h2>

  {#if isEditing}
    <form on:submit|preventDefault={handleSaveItem}>
      <div class="input-group">
        <label for="editItemName">Item Name</label>
        <input id="editItemName" type="text" bind:value={currentItemName} required/>
      </div>

      <div class="input-group">
        <label for="editItemPrice">Price</label>
        <input id="editItemPrice" type="number" step="0.01" min="0" bind:value={currentItemPrice} required />
      </div>

      <div class="input-group">
        <label for="editItemStock">Stock</label>
        <input id="editItemStock" type="number" min="0" bind:value={currentItemStock} required />
      </div>

      <div class="action-buttons">
        <button type="submit" class="save-button">Save Changes</button>
        <button type="button" class="delete-button" on:click={handleDeleteItem}>Delete Item</button>
      </div>
    </form>
  {:else}
    <p class="initial-message">Ready to add some new inventory? Click the button below!</p>
    <button class="add-new-item-button" on:click={handleOpenAddModal}>Add Item</button>
  {/if}
</div>

<AddItemModal
  bind:showModal={showAddModal}
  on:confirm={handleConfirmAdd}
  on:cancel={handleCancelAdd}
/>

<style>
  .add-edit-section {
    padding: 2rem;
    border-radius: 40px;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    height: 100%;
    box-sizing: border-box;
  }

  .glassy-background {
    background: rgba(44, 44, 44, 0.5);
    backdrop-filter: blur(10px) saturate(180%);
    -webkit-backdrop-filter: blur(10px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.15);
  }

  .section-title {
    font-size: 1.8em;
    font-weight: 600;
    color: #fff;
    text-align: center;
    margin-bottom: 1rem;
  }

  .initial-message {
    text-align: center;
    color: #ccc;
    font-size: 1.1em;
    margin-bottom: 2rem;
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .input-group {
    text-align: left;
    margin-bottom: 0.5rem;
  }

  .input-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-size: 0.9em;
    color: #ccc;
    font-weight: 500;
  }

  input {
    width: 100%;
    padding: 0.8em 1.2em;
    border-radius: 40px; 
    border: 1px solid rgba(255, 255, 255, 0.092);
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #ffffff;
    background-color: #0f0f0f98;
    transition: border-color 0.25s, box-shadow 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    box-sizing: border-box;
  }

  input:focus {
    border-color: #fff;
  }

  .action-buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin-top: 1.5rem;
  }

  button {
    padding: 0.7em 1.5em;
    border-radius: 40px;
    border: 1px solid transparent;
    font-size: 1em;
    font-weight: 600;
    font-family: inherit;
    cursor: pointer;
    transition: background-color 0.3s ease, border-color 0.25s ease, box-shadow 0.25s ease;
  }

  .add-new-item-button {
    background-color: #A020F0;
    color: #ffffff;
    box-shadow: 0 4px 15px rgba(160, 32, 240, 0.4);
    width: fit-content;
    margin: 0 auto;
    padding: 1em 2em;
    font-size: 1.1em;
    border-radius: 40px;
  }
  .add-new-item-button:hover {
    background-color: #8a00d9;
    border-color: #a020f0;
    box-shadow: 0 4px 20px rgba(160, 32, 240, 0.6);
  }
  .add-new-item-button:active {
    background-color: #6a00a0;
  }

  .save-button {
    background-color: #8a00d9;
    color: #ffffff;
    border-color: #a020f0;
    box-shadow: 0 4px 20px rgba(160, 32, 240, 0.6);
  }
  .save-button:hover {
    background-color: #6a00a0;
  }


  .delete-button {
    background-color: #dc3545; /* A decisive red for deleting! */
    color: #ffffff;
    box-shadow: 0 2px 8px rgba(220, 53, 69, 0.3);
  }
  .delete-button:hover {
    background-color: #c82333;
    border-color: #dc3545;
    box-shadow: 0 2px 10px rgba(220, 53, 69, 0.4);
  }
  .delete-button:active {
    background-color: #bd2130;
  }
</style>