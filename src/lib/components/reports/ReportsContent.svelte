<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  let salesTransactions = [
    { id: 'TRN001', date: '2025-06-15', totalPaid: 150.75, items: ['yummy chocolate', 'bikini tops'] },
    { id: 'TRN002', date: '2025-06-15', totalPaid: 25.00, items: ['cereals'] },
    { id: 'TRN003', date: '2025-06-14', totalPaid: 1313.00, items: ['my ass'] },
    { id: 'TRN004', date: '2025-06-14', totalPaid: 870.00, items: ['coffee machine'] },
    { id: 'TRN005', date: '2025-06-13', totalPaid: 9.99, items: ['uwu'] },
    { id: 'TRN006', date: '2025-06-13', totalPaid: 299.99, items: ['bikini tops'] },
    { id: 'TRN007', date: '2025-06-12', totalPaid: 12.99, items: ['yummy chocolate'] },
    { id: 'TRN008', date: '2025-06-12', totalPaid: 35.50, items: ['cereals'] },
    { id: 'TRN009', date: '2025-06-11', totalPaid: 18.25, items: ['milk frother'] },
    { id: 'TRN010', date: '2025-06-11', totalPaid: 50.00, items: ['bag of cookies'] },
    { id: 'TRN011', date: '2025-06-10', totalPaid: 12.99, items: ['pastry assortment'] },
    { id: 'TRN012', date: '2025-06-09', totalPaid: 8.50, items: ['organic green tea'] },
    { id: 'TRN013', date: '2025-06-09', totalPaid: 299.99, items: ['espresso machine'] },
    { id: 'TRN014', date: '2025-06-08', totalPaid: 5.00, items: ['ceramic mug'] },
    { id: 'TRN015', date: '2025-06-08', totalPaid: 35.50, items: ['deluxe coffee blend'] },
  ];

  // ✨ MOCK DATA SLAY for Product Performance & Low Stock ✨
  let allProducts = [
    { id: 'PROD001', name: 'Deluxe Coffee Blend', salesCount: 250, revenue: 3247.50, stock: 50 },
    { id: 'PROD002', name: 'Organic Green Tea (Box)', salesCount: 180, revenue: 1530.00, stock: 120 },
    { id: 'PROD003', name: 'Espresso Machine', salesCount: 15, revenue: 4499.85, stock: 5 }, // Low stock!
    { id: 'PROD004', name: 'Pastry Assortment', salesCount: 300, revenue: 1500.00, stock: 30 },
    { id: 'PROD005', name: 'Ceramic Mug (Large)', salesCount: 200, revenue: 1998.00, stock: 75 },
    { id: 'PROD006', name: 'Milk Frother', salesCount: 50, revenue: 1775.00, stock: 20 },
    { id: 'PROD007', name: 'Bag of Cookies', salesCount: 10, revenue: 32.50, stock: 4 }, // Low stock!
    { id: 'PROD008', name: 'Gift Card ($25)', salesCount: 100, revenue: 2500.00, stock: 150 },
    { id: 'PROD009', name: 'Hand Sanitizer', salesCount: 5, revenue: 25.00, stock: 3 },
    { id: 'PROD010', name: 'Hand Sanitizer (', salesCount: 5, revenue: 25.00, stock: 3 },
    { id: 'PROD011', name: 'Hand Sanitize', salesCount: 5, revenue: 25.00, stock: 3 },
    { id: 'PROD012', name: 'Hand Sanitizer (La', salesCount: 5, revenue: 25.00, stock: 3 },
    { id: 'PROD013', name: 'Hand Sanitizer (Lar', salesCount: 5, revenue: 25.00, stock: 3 },
    { id: 'PROD014', name: 'Hand Sanitizer (Large', salesCount: 5, revenue: 25.00, stock: 3 },
  ];

  let transactionSearchTerm = '';
  let filteredTransactions = salesTransactions;

  $: {
    if (transactionSearchTerm) {
      filteredTransactions = salesTransactions.filter(transaction =>
        transaction.id.toLowerCase().includes(transactionSearchTerm.toLowerCase()) ||
        transaction.date.includes(transactionSearchTerm) || // Allow searching by date
        String(transaction.totalPaid).includes(transactionSearchTerm) // Allow searching by total paid
      );
    } else {
      filteredTransactions = salesTransactions;
    }
  }

  // ✨ NEW: Product Search Term and Filtered Products ✨
  let productSearchTerm = '';
  let filteredProducts = allProducts;

  $: {
    if (productSearchTerm) {
      filteredProducts = allProducts.filter(product =>
        product.name.toLowerCase().includes(productSearchTerm.toLowerCase()) ||
        product.id.toLowerCase().includes(productSearchTerm.toLowerCase()) ||
        String(product.salesCount).includes(productSearchTerm) ||
        String(product.revenue).includes(productSearchTerm) ||
        String(product.stock).includes(productSearchTerm)
      ).sort((a, b) => b.salesCount - a.salesCount); // Keep sorting even with search
    } else {
      filteredProducts = allProducts.sort((a, b) => b.salesCount - a.salesCount);
    }
  }

  // Reactive calculations for Daily/Weekly/Monthly Revenue
  let dailyRevenue = 0;
  let weeklyRevenue = 0;
  let monthlyRevenue = 0;

  $: {
    const today = new Date().toISOString().slice(0, 10); //YYYY-MM-DD
    const oneWeekAgo = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString().slice(0, 10);
    const oneMonthAgo = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().slice(0, 10); // Approx. 30 days

    dailyRevenue = salesTransactions
      .filter(t => t.date === today)
      .reduce((sum, t) => sum + t.totalPaid, 0);

    weeklyRevenue = salesTransactions
      .filter(t => t.date >= oneWeekAgo && t.date <= today)
      .reduce((sum, t) => sum + t.totalPaid, 0);

    monthlyRevenue = salesTransactions
      .filter(t => t.date >= oneMonthAgo && t.date <= today)
      .reduce((sum, t) => sum + t.totalPaid, 0);
  }

  // Low Stock Alerts (based on allProducts, not filteredProducts)
  let lowStockProducts = allProducts.filter(p => p.stock < 5); // Below 5 as specified!
</script>

<div class="reports-container-glassy">
  <h1 class="page-title">Reports Dashboard</h1>

  <section class="reports-section sales-history-section">
    <h2 class="section-title">Sales History</h2>
    <input
      type="text"
      placeholder="Search transactions by ID, date, or total..."
      bind:value={transactionSearchTerm}
      class="search-bar"
    />

    <div class="transaction-list product-grid">
      {#each filteredTransactions as transaction (transaction.id)}
        <div class="transaction-card product-card">
          <h3>ID: {transaction.id}</h3>
          <p class="transaction-date">Date: {transaction.date}</p>
          <p class="transaction-total">Total: ${transaction.totalPaid.toFixed(2)}</p>
        </div>
      {:else}
        <p class="no-results">No transactions found, bestie!</p>
      {/each}
    </div>
  </section>

  <section class="reports-section revenue-reports-section">
    <h2 class="section-title">Revenue Overview</h2>
    <div class="revenue-cards-grid">
      <div class="revenue-card">
        <h3>Daily Revenue</h3>
        <p class="revenue-amount">${dailyRevenue.toFixed(2)}</p>
      </div>
      <div class="revenue-card">
        <h3>Weekly Revenue</h3>
        <p class="revenue-amount">${weeklyRevenue.toFixed(2)}</p>
      </div>
      <div class="revenue-card">
        <h3>Monthly Revenue</h3>
        <p class="revenue-amount">${monthlyRevenue.toFixed(2)}</p>
      </div>
    </div>
  </section>

  <section class="reports-section product-performance-section">
    <h2 class="section-title">Product Performance</h2>
    <input
      type="text"
      placeholder="Search products by name, ID, sales, or revenue..."
      bind:value={productSearchTerm}
      class="search-bar"
    />
    <div class="product-performance-list product-grid">
      {#each filteredProducts as product (product.id)}
        <div class="performance-card product-card">
          <h3>{product.name}</h3>
          <p>Sold: {product.salesCount}</p>
          <p>Revenue: ${product.revenue.toFixed(2)}</p>
          <p>Stock: {product.stock}</p>
        </div>
      {:else}
        <p class="no-results">No product data to display!</p>
      {/each}
    </div>
  </section>

  <section class="reports-section">
    <h2 class="section-title">Low Stock Alerts</h2>
    <div class="low-stock-list product-grid">
      {#each lowStockProducts as product (product.id)}
        <div class="low-stock-card product-card alert-card">
          <h3>{product.name}</h3>
          <p>Current Stock: {product.stock}</p>
        </div>
      {:else}
        <p class="no-results">No low stock alerts! Everything's stocked up, bestie!</p>
      {/each}
    </div>
  </section>
</div>

<style>
  /* All existing styles are kept exactly as you provided them! */
  .reports-container-glassy {
    background: rgba(44, 44, 44, 0.5);
    border-radius: 40px;
    padding: 1.5rem;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);

    width: 100%;
    height: 100vh;
    color: #fff;

    backdrop-filter: blur(10px) saturate(180%);
    -webkit-backdrop-filter: blur(10px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.15);


    overflow-y: auto;
    overflow-x: hidden;

    /* Custom scrollbar for this container */
    &::-webkit-scrollbar-track {
      background: transparent;
    }
    &::-webkit-scrollbar-thumb {
      background-color: rgba(255, 255, 255, 0.2);
      border-radius: 10px;
      border: 2px solid transparent;
    }
    &::-webkit-scrollbar {
      width: 10px;
      height: 10px;
    }
    &::-webkit-scrollbar-thumb:hover {
      background-color: rgba(255, 255, 255, 0.4);
    }
  }

  .page-title {
    font-size: 2.5rem;
    font-weight: 700;
    color: #fff;
    margin-bottom: 2rem;
    text-align: center;
  }

  /* Reports Section Styling (similar to settings-section) */
  .reports-section {
    padding-bottom: 2rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    height: fit-content;
    overflow-y: auto; /* Added back for individual section scrolling, if needed */
  }

  .reports-section:last-of-type {
    border-bottom: none;
    margin-bottom: 0;
    padding-bottom: 0;
  }

  .section-title {
    font-size: 1.8rem;
    font-weight: 600;
    color: #fff;
    margin-bottom: 1.5rem;
    padding-bottom: 0.5rem;
  }

  /* Shared Search Bar Styles (copied from product list) */
  .search-bar {
    width: 100%;
    padding: 0.8rem 1rem;
    margin-bottom: 1.5rem;
    border-radius: 40px;
    font-size: 1rem;
    box-sizing: border-box;
    font-family: 'goia regular', sans-serif; /* Added sans-serif fallback */
    background-color: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.092); /* Base border */
    color: #fff;
    outline: none; /* Removed default browser outline */
    transition: border 0.2s ease, box-shadow 0.2s ease; /* Smooth transition */
  }

  .search-bar::placeholder {
    color: rgba(255, 255, 255, 0.5);
    font-family: 'goia regular', sans-serif;
  }

  .search-bar:focus {
    border-color: #ffffff;
  }


  .product-grid {
    min-height: 10rem;
    max-height: 50rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
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
      width: 5px;
      height: 10px;
    }
    &::-webkit-scrollbar-thumb:hover {
      background-color: rgba(255, 255, 255, 0.4);
    }
  }

  /* Shared Product Card/List Item Styles */
  .product-card { /* Base for transaction cards, performance cards, low stock cards */
    background: rgba(44, 44, 44, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.092);
    color: #fff;
    border-radius: 40px;
    padding-left: 1rem;
    padding-right: 1rem;
    cursor: pointer; /* Keep cursor pointer for hover feedback */
    transition: transform 0.2s ease, box-shadow 0.2s ease, background 0.4s ease; /* Added background to transition */
    display: grid; /* Use grid for internal layout */
    align-items: center;
    width: 95%; /* Take 95% width of parent */
    height: auto; /* Height adapts to content */
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

  .no-results {
    text-align: center;
    font-style: italic;
    grid-column: 1 / -1; /* Center across grid columns if in a grid */
  }

  /* --- Specific Styles for Reports Content --- */

  /* Transaction List specific overrides/additions */
  .transaction-card {
    grid-template-columns: 1fr auto auto; /* ID, Date, Total Paid */
    gap: 1.5rem; /* Slightly less gap than product card for tighter info *//* Adjusted padding for better fit */
  }
  .transaction-total {
    font-size: 1.25rem;
    font-weight: bold;
    color: #ffffff;
  }

  /* Revenue Cards Grid */
  .revenue-cards-grid {
    display: flex;
    flex-wrap: wrap; /* Allows cards to wrap to next line */
    justify-content: center; /* Center cards horizontally */
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .revenue-card {
    background: rgba(44, 44, 44, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.092);
    border-radius: 40px; /* Slightly less rounded than product cards */
    padding: 1rem 1.5rem;
    flex: 1 1 calc(33% - 1rem); /* Distribute space for 3 cards per row, adjust as needed */
    max-width: calc(33% - 1rem); /* Max width for 3 columns */
    text-align: center;
    min-width: 150px; /* Ensure cards don't get too small */
  }

  .revenue-card h3 {
    font-size: 1.2rem;
    margin-bottom: 0.5rem;
    color: #ccc;
  }

  .revenue-card .revenue-amount {
    font-size: 2rem;
    font-weight: bold;
    color: #ffffff; /* Green for positive revenue! Slay! */
  }

  /* Product Performance Cards */
  .performance-card {
    grid-template-columns: 2fr 1fr 1fr 0.5fr; /* Name, Sold, Revenue, Stock */
    gap: 1rem;
  }
  .performance-card p {
    font-size: 0.95rem;
    color: #ccc;
  }

  /* Low Stock Alert Cards */
  .low-stock-card {
    grid-template-columns: 2fr 1fr; /* Name, Current Stock */
    gap: 1.5rem;
    cursor: default; /* No pointer for alerts */
  }

  .low-stock-card.alert-card {
    background: rgba(220, 53, 69, 0.5); /* Reddish background for alerts! */
    border-color: rgba(255, 99, 71, 0.3); /* Slightly brighter border */
    box-shadow: 0 0 10px rgba(220, 53, 69, 0.4); /* Glowing alert! */
  }
  .low-stock-card.alert-card:hover {
    background: rgba(255, 99, 71, 0.6); /* Even brighter on hover */
    backdrop-filter: blur(5px);
  }
  .low-stock-card h3 {
    color: #fff; /* Ensure text is white */
  }
  .low-stock-card p {
    color: #fff;
    font-weight: bold;
  }

</style>