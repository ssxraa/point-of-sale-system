<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import ProductList from '$lib/components/pos/productlist.svelte';
  import ShoppingCart from '$lib/components/pos/shoppingcart.svelte';
  import type { CartItem } from '$lib/components/pos/shoppingcart.svelte'; // Import the CartItem type

  // Define a minimal StoreSettings type needed for the receipt data
  // This helps avoid assuming other fields from the full SettingsContent.svelte type
  interface StoreSettings {
    store_name: string;
    store_address: string;
    store_phone: string;
    receipt_header: string;
    receipt_footer: string;
    // Add any other settings fields here that you intend to use in the receipt data
  }

  // Define a type for the Product items coming from Rust's get_inventory
  // Ensure the 'id' type matches Rust's i64 (which translates to number in JS/TS)
  interface Product {
    id: number; // Assuming Rust's i64 for product ID
    name: string;
    price: number;
    stock: number;
  }

  let products: Product[] = []; // Use the defined Product type
  let cart: CartItem[] = [];

  let errorMessage = "";
  let loading = true;
  let storeSettings: StoreSettings | null = null; // State to hold loaded settings

  async function loadProductsAndSettings() {
    loading = true;
    errorMessage = "";
    try {
      products = await invoke<Product[]>("get_inventory"); // Explicitly type the return
      storeSettings = await invoke<StoreSettings>("load_settings"); // Load settings too
    } catch (err: any) {
      errorMessage = `Failed to load data: ${err.message || JSON.stringify(err)}`;
      console.error('Loading error:', err);
    }
    loading = false;
  }

  onMount(loadProductsAndSettings); // Call the combined loading function

  // --- CART MANAGEMENT FUNCTIONS ---
  function addToCart(product: Product) {
    const existingItem = cart.find(item => item.id === product.id);

    // Check if there's enough stock
    // This is client-side validation, backend will also enforce.
    const currentStock = products.find(p => p.id === product.id)?.stock || 0;
    const currentCartQuantity = existingItem ? existingItem.quantity : 0;

    if (currentCartQuantity + 1 > currentStock) {
      errorMessage = `Girl, we only have ${currentStock} of ${product.name} left in stock! 😩`;
      setTimeout(() => errorMessage = '', 3000);
      return;
    }

    if (existingItem) {
      existingItem.quantity += 1;
    } else {
      cart = [...cart, { id: product.id, name: product.name, price: product.price, quantity: 1 }];
    }
    cart = cart; // Trigger Svelte reactivity
  }

  function removeFromCart(productId: number) {
    cart = cart.filter(item => item.id !== productId);
  }

  function decreaseQuantity(productId: number) {
    const existingItem = cart.find(item => item.id === productId);
    if (existingItem && existingItem.quantity > 1) {
      existingItem.quantity -= 1;
      cart = cart; // Trigger Svelte reactivity
    } else {
      removeFromCart(productId); // Remove if quantity is 1 and decreased
    }
  }

  function handleBarcodeScan(event: Event) {
    const inputElement = event.target as HTMLInputElement;
    const scannedProductId = inputElement.value.trim();
    if (!scannedProductId) return;

    const scannedIdNum = Number(scannedProductId);
    if (isNaN(scannedIdNum)) {
        errorMessage = 'Invalid barcode format. Please enter a number.';
        setTimeout(() => errorMessage = '', 3000);
        inputElement.value = ''; // Clear invalid input
        return;
    }

    const productToScan = products.find(p => p.id === scannedIdNum);

    if (productToScan) {
      addToCart(productToScan);
      inputElement.value = ''; // Clear the input after successful scan
    } else {
      errorMessage = 'Product not found, sis! Check that ID! 😩';
      setTimeout(() => errorMessage = '', 3000);
      inputElement.value = ''; // Clear input for not found
    }
  }

  // --- NEW: Function to handle checkout request from ShoppingCart component ---
  async function handleCheckoutRequest(event: CustomEvent<CartItem[]>) {
    const itemsToCheckout = event.detail; // This is the cart data from ShoppingCart
    errorMessage = ""; // Clear previous errors

    if (itemsToCheckout.length === 0) {
      errorMessage = "Can't checkout an empty cart, honey! 🛍️";
      setTimeout(() => errorMessage = '', 3000);
      return;
    }

    try {
      // 1. Invoke the Rust backend's checkout command
      // The `checkout` command in pos.rs returns a SalesTransaction
      const salesTransaction = await invoke<any>("checkout", { items: itemsToCheckout });
      console.log('Checkout successful, darling!', salesTransaction);

      // 2. Prepare receipt data for printing
      if (storeSettings) { // Ensure settings are loaded
          const receiptItems = itemsToCheckout.map(item => ({
              name: item.name,
              quantity: item.quantity,
              price: item.price,
              total: item.price * item.quantity // Calculate total for each item
          }));

          // The total_paid and subtotal should come from the salesTransaction returned by the backend
          const totalPaid = salesTransaction.total_paid;
          const subtotal = itemsToCheckout.reduce((sum, item) => sum + (item.price * item.quantity), 0);
          // Assuming taxes were applied to reach totalPaid, or you re-calculate them here if needed for receipt breakdown
          const taxes = 0; // This assumes totalPaid already includes taxes from Rust side

          // For a real POS, tenderAmount would come from a payment input (e.g., cash tendered)
          // For now, let's assume tender_amount equals total_paid (exact change) for simplicity
          const tenderAmount = totalPaid; // Placeholder: In a real app, this would be user input
          const changeDue = tenderAmount - totalPaid;

          const receiptData = {
              store_name: storeSettings.store_name,
              store_address: storeSettings.store_address,
              store_phone: storeSettings.store_phone,
              receipt_header: storeSettings.receipt_header,
              receipt_footer: storeSettings.receipt_footer,
              items: receiptItems,
              total_paid: subtotal,
              tender_amount: tenderAmount,
              change_due: changeDue,
              transaction_id: salesTransaction.id,
              transaction_date: salesTransaction.date, // Use the date returned by backend
          };

          // 3. Invoke the print_receipt command with the prepared data
          await invoke("print_receipt", { receiptData });
          console.log('Receipt print requested, sweetie! 📄');
      } else {
          console.warn("Store settings not loaded, skipping receipt printing.");
      }

      clearCart(); // Only clear cart on successful backend checkout AND print request
      // After a successful sale, reload products to get updated stock
      await loadProductsAndSettings(); // Refresh inventory data after checkout
      errorMessage = "Sale completed and receipt printed! ✨";
      setTimeout(() => errorMessage = '', 3000); // Clear success message

    } catch (err: any) {
      errorMessage = `Checkout failed, boo! Error: ${err.message || JSON.stringify(err)}`;
      console.error('Checkout error:', err);
      // Optional: If checkout fails due to stock, you might want to reload products to reflect current stock
      await loadProductsAndSettings();
    }
  }

  function clearCart() {
    cart = [];
  }
</script>

<div class="pos-layout">
  {#if loading}
    <p>Loading products...</p>
  {:else}
    {#if errorMessage}
      <p style="color: red;">{errorMessage}</p>
    {/if}
    <ProductList products={products} on:productSelected={(event) => addToCart(event.detail)} />
   <ShoppingCart
      cart={cart}
      handleBarcodeScan={handleBarcodeScan}
      on:itemAdded={(event) => addToCart(event.detail)}
      on:itemRemoved={(event) => removeFromCart(event.detail)}
      on:quantityDecreased={(event) => decreaseQuantity(event.detail)}
      on:requestCheckout={handleCheckoutRequest} 
    />
  {/if}
</div>

<style>
  .pos-layout {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 1.5rem;
    padding: 0;
    background-color: none;
    border-radius: 40px;
    overflow: hidden;
  }
</style>