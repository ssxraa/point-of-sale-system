<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  // Define a type for a cart item that matches what your Rust backend expects
  // Assuming 'id' comes as a number (i64 in Rust maps to number in JS/TS)
  export type CartItem = {
    id: number;
    name: string;
    price: number;
    quantity: number;
  };

  export let cart: CartItem[] = [];
  export let handleBarcodeScan: (event: Event) => void;

  $: subtotal = cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);
  $: total = subtotal;

  const dispatch = createEventDispatcher();

  function emitAddToCart(item: CartItem) {
    dispatch('itemAdded', item);
  }
  function emitRemoveFromCart(productId: number) { // Changed to number
    dispatch('itemRemoved', productId);
  }
  function emitDecreaseQuantity(productId: number) { // Changed to number
    dispatch('quantityDecreased', productId);
  }

  let showCheckoutModal = false;

  function finalizeCheckout() {
    if (cart.length === 0) {
      alert('Your cart is empty, bestie! Add some items first! 🛒');
      return;
    }
    // Instead of just showing modal, now we dispatch the request to the parent (+page.svelte)
    // The parent will then handle the backend call and decide to show the modal or print.
    // We will now show the modal directly from this component anymore unless the parent confirms checkout
    showCheckoutModal = true;
  }

  function confirmCheckout() {
    // This function will now dispatch the cart data to the parent for backend processing
    // It will be called from within the modal's confirm button
    dispatch('requestCheckout', cart); // NEW: Dispatch 'requestCheckout' with the entire cart
    showCheckoutModal = false; // Close the modal
    // The actual cart clearing (dispatch('checkoutFinalized')) will be handled by the parent
    // AFTER the backend call is successful and potentially after printing.
  }

  function cancelCheckout() {
    showCheckoutModal = false;
  }

  function printReceipt() {
    // This function should be called by the parent after backend checkout and data processing
    // For now, it will trigger the print directly from the modal, but the actual printing logic
    // is in pos/+page.svelte after the backend call.
    alert('Printing receipt... 🖨️');
    window.print(); // This triggers browser print dialog, not the Tauri backend command
    confirmCheckout(); // Still confirm the checkout after printing
  }
</script>

<section class="shopping-cart-section">
  <h2>Shopping Cart</h2>

  <input
    type="text"
    placeholder="Scan barcode or enter product ID..."
    on:change={handleBarcodeScan}
    class="barcode-scanner-input"
  />

  {#if cart.length > 0}
    <div class="cart-items">
      {#each cart as item (item.id)}
        <div class="cart-item">
          <span class="item-name">{item.name}</span>
          <span class="item-qty">x{item.quantity}</span>
          <span class="item-price">{(item.price * item.quantity).toFixed(2)}Da</span>
          <div class="item-actions">
            <button on:click={() => emitDecreaseQuantity(item.id)}>-</button>
            <button on:click={() => emitAddToCart(item)}>+</button>
            <button on:click={() => emitRemoveFromCart(item.id)} class="remove-btn">x</button>
          </div>
        </div>
      {/each}
    </div>

    <div class="cart-summary">
      <div class="summary-line total-line"><span>Total:</span><span>{total.toFixed(2)}Da</span></div>
    </div>

    <button class="checkout-btn" on:click={finalizeCheckout}>Finalize Checkout</button>
  {:else}
    <p class="empty-cart-message">Your cart is empty, add some fabulous items! 🛒</p>
  {/if}
</section>

{#if showCheckoutModal}
  <div class="modal-overlay" on:click={cancelCheckout}>
    <div class="modal-content" on:click|stopPropagation>
      <h3>Confirm Checkout</h3>
      <p>Total Amount: <span class="modal-total">${total.toFixed(2)}</span></p>
      <div class="modal-actions">
        <button on:click={confirmCheckout}>Confirm (No Print)</button>
        <button on:click={printReceipt}>Confirm & Print Receipt</button>
        <button on:click={cancelCheckout} class="cancel-btn">Cancel</button>
      </div>
    </div>
  </div>
{/if}

<style>
  /* Copy all relevant CSS from .shopping-cart-section, .barcode-scanner-input,
     .cart-items, .cart-item, etc., and all modal styles from your +page.svelte */

  .shopping-cart-section {
    background: rgb(44, 44, 44, 0.5);
    backdrop-filter: blur(10px) saturate(180%);
    -webkit-backdrop-filter: blur(10px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: #fff;
    border-radius: 40px;
    padding: 1.5rem;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.05);
    display: flex;
    flex-direction: column;
    position: relative;
    height: calc(100vh - 5rem);
    overflow: hidden;
  }

  .shopping-cart-section h2 {
    font-size: 1.8rem;
    margin-bottom: 1rem;
    text-align: center;
  }

  .barcode-scanner-input {
    width: 100%;
    background: none;
    border: #e0e0e0 1px solid;
    padding: 0.8rem 1rem;
    margin-bottom: 1.5rem;
    border: 1px solid rgba(255, 255, 255, 0.092);
    border-radius: 40px;
    font-size: 1rem;
    box-sizing: border-box;
    color: #fff;
  }
  .barcode-scanner-input::placeholder {
    color: #ffffff;
  }

  .cart-items {
    flex-grow: 1;
    overflow-y: auto;
    padding-right: 5px;
        &::-webkit-scrollbar-track {
      background: transparent;
    }

    &::-webkit-scrollbar-thumb {
      background-color: rgba(255, 255, 255, 0.2);
      border-radius: 10px;
      border: 2px solid transparent;
    }

    &::-webkit-scrollbar {
      width: 7px;
      height: 7px;
    }

    &::-webkit-scrollbar-thumb:hover {
      background-color: rgba(255, 255, 255, 0.4);
    }
  }

  .cart-item {
    display: grid;
    grid-template-columns: 3fr 0.5fr 1fr 1.5fr;
    gap: 0.5rem;
    align-items: center;
    padding: 0.75rem 0;
    border-bottom: 1px dashed #eee;
    font-size: 0.95rem;
    
  }

  .cart-item:last-child {
    border-bottom: none;
  }

  .item-name {
    font-weight: 500;
    word-break: break-word;
  }
  .item-qty {
    text-align: center;
  }
  .item-price {
    text-align: right;
    font-weight: bold;
    color: #ffffff;
  }
  .item-actions {
    display: flex;
    gap: 0.2rem;
    justify-content: flex-end;
  }
  .item-actions button {
    background: #f0f0f0;
    border: 1px solid #ddd;
    border-radius: 50%;
    padding: 0.2rem 0.4rem;
    cursor: pointer;
    font-size: 0.8rem;
    line-height: 1;
    transition: background 0.2s;
    width: 20px;
    height: 20px;
  }
  .item-actions button:hover {
    background: #e0e0e0;
  }
  .item-actions .remove-btn {
    background: #dc3545;
    color: #fff;
    border-color: #dc3545;
  }
  .item-actions .remove-btn:hover {
    background: #c82333;
  }

  .empty-cart-message {
    text-align: center;
    color: #ffffff;
    padding: 2rem 0;
  }

  .cart-summary {
    padding-top: 1rem;
    margin-top: 1rem;
  }
  .summary-line {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.5rem;
    font-size: 1rem;
  }
  .summary-line span:first-child {
    font-weight: normal;
  }
  .summary-line span:last-child {
    font-weight: bold;
    color: #ffffff;
  }
  .total-line {
    font-size: 1.3rem;
    font-weight: bold;
    border-top: 1px solid #eee;
    padding-top: 0.75rem;
    margin-top: 0.75rem;
  }

  .checkout-btn {
    width: 100%;
    padding: 1rem;
    background-color: #A020F0;
    box-shadow: 0 4px 15px rgba(160, 32, 240, 0.4);
    color: #fff;
    border: none;
    border-radius: 40px;
    font-size: 1.2rem;
    font-weight: bold;
    cursor: pointer;
    transition: background-color 0.3s ease, transform 0.2s ease;
    margin-top: 1.5rem;
    font-family: 'boya';
  }
  .checkout-btn:hover {
    background-color: #8a00d9;
    border-color: #a020f0;
    box-shadow: 0 4px 20px rgba(160, 32, 240, 0.6);
  }

  /* Modal Styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.6);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    background-color: rgb(44, 44, 44, 0.5);
    backdrop-filter: blur(10px) saturate(180%);
    -webkit-backdrop-filter: blur(10px) saturate(180%);
    padding: 2rem;
    border-radius: 40px;
    box-shadow: 0 5px 15px rgba(0, 0, 0, 0.3);
    text-align: center;
    max-width: 400px;
    width: 90%;
  }

  .modal-content h3 {
    font-size: 1.8rem;
    color: #ffffff;
    margin-bottom: 1.5rem;
  }

  .modal-total {
    font-size: 2rem;
    font-weight: bold;
    color: #ffffff;
    margin-top: 0.5rem;
    display: block;
  }

  .modal-actions {
    margin-top: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .modal-actions button {
    padding: 0.8rem 1.5rem;
    border: none;
    border-radius: 40px;
    font-size: 1rem;
    font-weight: bold;
    cursor: pointer;
    transition: background-color 0.2s ease;
    font-family: 'boya';
  }

  .modal-actions button:first-child {
    background-color: #A020F0;
    box-shadow: 0 4px 15px rgba(160, 32, 240, 0.4);
    color: #fff;
  }
  .modal-actions button:first-child:hover {
    background-color: #8a00d9;
    border-color: #a020f0;
    box-shadow: 0 4px 20px rgba(160, 32, 240, 0.6);
  }

  .modal-actions button:nth-child(2) {
    background-color: #A020F0;
    box-shadow: 0 4px 15px rgba(160, 32, 240, 0.4);
    color: #fff;
  }
  .modal-actions button:nth-child(2):hover {
    background-color: #8a00d9;
    border-color: #a020f0;
    box-shadow: 0 4px 20px rgba(160, 32, 240, 0.6);
  }

  .modal-actions .cancel-btn {
    background-color: #6c757d;
    color: #fff;
  }
  .modal-actions .cancel-btn:hover {
    background-color: #5a6268;
  }
</style>