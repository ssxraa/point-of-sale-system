<script>
import { page } from '$app/state';
import FaHome from 'svelte-icons/fa/FaHome.svelte';
import FaShoppingCart from 'svelte-icons/fa/FaShoppingCart.svelte';
import FaBoxOpen from 'svelte-icons/fa/FaBoxOpen.svelte';
import FaChartLine from 'svelte-icons/fa/FaChartLine.svelte';
import FaCog from 'svelte-icons/fa/FaCog.svelte';
import FaTh from 'svelte-icons/fa/FaTh.svelte';
</script>

<aside class="sidebar">
  <nav>
    <ul>
      <li><a href="/" class:active={page.url.pathname === '/'}><FaHome /></a></li>
      <li><a href="/pos" class:active={page.url.pathname.startsWith('/pos')}> <FaShoppingCart /> </a></li>
      <li><a href="/inventory" class:active={page.url.pathname.startsWith('/inventory')}><FaBoxOpen /></a></li>
      <li><a href="/reports" class:active={page.url.pathname.startsWith('/reports')}><FaChartLine /></a></li>
      <li><a href="/settings" class:active={page.url.pathname.startsWith('/settings')}><FaCog /></a></li>
    </ul>
  </nav>
</aside>

<style>
.sidebar {
  border-radius: 50px;
  overflow-y: auto;
  width: 67px;
  background: rgb(44, 44, 44, 0.5);
  backdrop-filter: blur(10px) saturate(180%);
  -webkit-backdrop-filter: blur(10px) saturate(180%);
  border: 1px solid rgba(255, 255, 255, 0.092);
  color: #fff;
  height: calc(100vh - 4rem);
  position: fixed;
  left: 0;
  top: 0;
  padding-top: 2rem;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1); /* Subtle shadow for depth */
  margin: 1rem;

}
.sidebar ul {
  list-style: none;
  padding: 0;
}
.sidebar li {
  margin: 1rem 0;
}
.sidebar a {
  color: #fff;
  text-decoration: none;
  padding: 0.5rem 1rem;
  display: block;
  transition: background 0.2s;
}

.sidebar a :global(svg) {
  width: 55%;
  height: 55%;
  fill: currentColor;
  display: block;
  margin: 0 auto;
}

.sidebar a.active,
.sidebar a:hover {
  background: rgba(60, 60, 60, 0.2); 
  backdrop-filter: blur(7px); 
  -webkit-backdrop-filter: blur(7px);
}
</style>