<script lang="ts">

  let itemName: string = '';
  let itemPrice: number | null = null; 
  let itemStock: number | null = null; 

  export let showModal: boolean = false;

  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  function handleConfirm() {
    if (!itemName.trim() || itemPrice === null || itemPrice < 0 || itemStock === null || itemStock < 0) {
      alert('Darling, all fields must be filled correctly! Name cannot be empty, and price/stock must be non-negative numbers. 😩');
      return;
    }
    dispatch('confirm', {
      name: itemName.trim(),
      price: itemPrice,
      stock: itemStock
    });
    resetForm();
    showModal = false;
  }

  function handleCancel() {
    dispatch('cancel');
    resetForm();
    showModal = false; 
  }

  function resetForm() {
    itemName = '';
    itemPrice = null;
    itemStock = null;
  }

  $: if (!showModal) {
    resetForm();
  }
</script>

{#if showModal}
  <div class="modal-overlay" on:click|self={handleCancel}>
    <div class="modal-content glassy-background">
      <h2 class="modal-title">Add New Item</h2>

      <form on:submit|preventDefault={handleConfirm}>
        <div class="input-group">
          <label for="itemName">Item Name</label>
          <input
            id="itemName"
            type="text"
            placeholder="E.g., Sparkling Water"
            bind:value={itemName}
            required
            
          />
        </div>

        <div class="input-group">
          <label for="itemPrice">Price</label>
          <input
            id="itemPrice"
            type="number"
            step="0.01"
            min="0"
            placeholder="E.g., 2.50"
            bind:value={itemPrice}
            required
          />
        </div>

        <div class="input-group">
          <label for="itemStock">Stock</label>
          <input
            id="itemStock"
            type="number"
            min="0"
            placeholder="E.g., 100"
            bind:value={itemStock}
            required
          />
        </div>

        <div class="modal-actions">
            <button type="submit" class="confirm-button">
            Confirm Add
          </button>
          <button type="button" class="cancel-button" on:click={handleCancel}>
            Cancel
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    padding: 2.5rem 3rem;
    border-radius: 40px; 
    text-align: center;
    max-width: 500px;
    width: 90%;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    position: relative;
  }

  .glassy-background {
    background: rgba(44, 44, 44, 0.5);
    backdrop-filter: blur(10px) saturate(180%);
    -webkit-backdrop-filter: blur(10px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.15);
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.5);
  }

  .modal-title {
    font-size: 2em;
    font-weight: 700;
    color: #fff;
    margin-bottom: 0.5rem;
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .input-group {
    text-align: left;
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
    border-radius: 40px; /* Slightly less rounded than your login, for more form-like */
    border: 1px solid rgba(255, 255, 255, 0.092);
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #ffffff;
    background-color: #0f0f0f98;
    transition: border-color 0.25s, box-shadow 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    box-sizing: border-box; /* Crucial for proper width */
  }

  input:focus {
    border-color:#fff;
  }

  .modal-actions {
    display: flex;
    flex-direction: column;
    justify-content: center; /* Push buttons to the right */
    gap: 1rem;
    margin-top: 1rem;
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

  .confirm-button {
    background-color: #A020F0; /* Your signature purple! */
    color: #ffffff;
    box-shadow: 0 2px 8px rgba(160, 32, 240, 0.3);
  }

  .confirm-button:hover {
    background-color: #8a00d9;
    border-color: #a020f0;
    box-shadow: 0 2px 10px rgba(160, 32, 240, 0.4);
  }
  .confirm-button:active {
    background-color: #6a00a0;
  }

  .cancel-button {
    background-color: #6c757d; /* A subtle grey for cancel */
    color: #ffffff;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  }

  .cancel-button:hover {
    background-color: #5a6268;
    border-color: #6c757d;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
  }
  .cancel-button:active {
    background-color: #495057;
  }
</style>