<script>
  import ProductList from '$lib/components/pos/productlist.svelte';
  import ShoppingCart from '$lib/components/pos/shoppingcart.svelte';

  // This `products` array can stay here for now if you prefer it central,
  // or move it into ProductList.svelte and fetch/manage it there.
  // For this example, let's keep a master list here for simplicity of adding to cart.
  let products = [
    { id: '1234567890128', name: 'yummy chocolate', price: 12.99, stock: 50 },
    { id: '1234567890135', name: 'booty shorts', price: 8.50, stock: 120 },
    { id: '1234567890142', name: 'bikini tops', price: 299.99, stock: 5 },
    { id: '1234567890159', name: 'my ass', price: 1313.00, stock: 1 },
    { id: '1234567890166', name: 'uwu', price: 9.99, stock: 87 },
    { id: '1234567890173', name: 'cereals', price: 35.50, stock: 20 },
    { id: '1234567890180', name: 'coffee machine', price: 870.00, stock: 100 },
    { id: '1234567890197', name: 'nothing (we take your money)', price: 25.00, stock: 40 },
  ];

  let cart = []; 

  // --- CART MANAGEMENT FUNCTIONS (Now centralized here) ---
  function addToCart(product) {
    const existingItem = cart.find(item => item.id === product.id);
    if (existingItem) {
      existingItem.quantity += 1;
    } else {
      cart = [...cart, { ...product, quantity: 1 }];
    }
    cart = cart; // Reassign to ensure reactivity
  }

  function removeFromCart(productId) {
    cart = cart.filter(item => item.id !== productId);
  }

  function decreaseQuantity(productId) {
    const existingItem = cart.find(item => item.id === productId);
    if (existingItem && existingItem.quantity > 1) {
      existingItem.quantity -= 1;
      cart = cart;
    } else {
      removeFromCart(productId);
    }
  }

  function handleBarcodeScan(event) {
    const scannedProductId = event.target.value.trim();
    if (!scannedProductId) return;

    // We need access to the full products list here to find the scanned item
    const productToScan = products.find(p => p.id === scannedProductId);

    if (productToScan) {
      addToCart(productToScan);
      event.target.value = ''; // Clear input after scan
    } else {
      alert('Product not found! 😩');
    }
  }

  function clearCart() {
    cart = [];
  }

</script>

<div class="pos-layout">
  <ProductList on:productSelected={(event) => addToCart(event.detail)} />

  <ShoppingCart
    cart={cart}
    handleBarcodeScan={handleBarcodeScan}
    on:itemAdded={(event) => addToCart(event.detail)}
    on:itemRemoved={(event) => removeFromCart(event.detail)}
    on:quantityDecreased={(event) => decreaseQuantity(event.detail)}
    on:checkoutFinalized={clearCart}
  />
</div>

<style>
  /* Keep only the .pos-layout styles here, as product-list-section and shopping-cart-section styles move to their components */
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