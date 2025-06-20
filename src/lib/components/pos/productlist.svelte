<script>
  // Mock data for now - later this will come from backend or props
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

  let searchTerm = '';
  let filteredProducts = products;

  $: {
    if (searchTerm) {
      filteredProducts = products.filter(product =>
        product.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        product.id.includes(searchTerm)
      );
    } else {
      filteredProducts = products;
    }
  }

  // ✨✨✨ SVELTE EVENT DISPATCHER SLAY ✨✨✨
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  function selectProduct(product) {
    // When a product is clicked, we dispatch an event to the parent
    dispatch('productSelected', product);
  }
</script>

<section class="product-list-section">
  <h2>Available Products</h2>
  <input
    type="text"
    placeholder="Search products by name or ID..."
    bind:value={searchTerm}
    class="search-bar"
  />

  <div class="product-grid">
    {#each filteredProducts as product (product.id)}
      <div class="product-card" on:click={() => selectProduct(product)}>
        <h3>{product.name}</h3>
        <p class="product-price">${product.price.toFixed(2)}</p>
        <p class="product-stock">Stock: {product.stock}</p>
      </div>
    {:else}
      <p class="no-results">No products found, bestie! 😩</p>
    {/each}
  </div>
</section>

<style>
  /* Copy all relevant CSS from .product-list-section, .search-bar,
     .product-grid, .product-card, .no-results from your +page.svelte */

  .product-list-section {
    background: none;
    color: #fff;
    border-radius: 40px;
    padding: 1.5rem;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.05);
    display: flex;
    flex-direction: column;
    height: calc(100vh - 5rem);
  }

  .product-list-section h2 {
    font-size: 1.8rem;
    margin-bottom: 1rem;
    text-align: center;
  }

  .search-bar {
    width: 100%;
    padding: 0.8rem 1rem;
    margin-bottom: 1.5rem;
    border-radius: 40px;
    font-size: 1rem;
    box-sizing: border-box;
    font-family: 'goia regular';
    background: rgb(44, 44, 44, 0.5);
    backdrop-filter: blur(10px) saturate(180%);
    -webkit-backdrop-filter: blur(10px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.092);
    color: #fff;
  }
  .search-bar::placeholder {
    color: #ffffff;
    font-family: 'goia regular';
  }

  
  .product-grid {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    flex-grow: 1;
    overflow-y: auto;
    padding-right: 5px;
  }
  

  .product-card {
    background: rgb(44, 44, 44, 0.5);
    backdrop-filter: blur(10px) saturate(180%);
    -webkit-backdrop-filter: blur(10px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.092);
    color: #fff;
    border-radius: 40px;
    padding-left: 1rem;
    padding-right: 1rem;
    cursor: pointer;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    display: grid;
    grid-template-columns: 1fr 1fr 90px;
    gap: 3rem;
    align-items: center;
    width: 95%;
    height: auto;
    transition: background 0.4s ease;
  }

  .product-card:hover {
    background: rgba(60, 60, 60, 0.6); 
    backdrop-filter: blur(7px); 
    -webkit-backdrop-filter: blur(7px);

  }

  .product-card h3 {
    font-size: 1.1rem;
    word-break: break-word;
  }

  .product-card .product-price {
    font-size: 1.25rem;
    font-weight: bold;
    color: #ffffff;
  }

  .product-card .product-stock {
    font-size: 0.9rem;
  }

  .no-results {
    text-align: center;
    font-style: italic;
    grid-column: 1 / -1;
  }
</style>