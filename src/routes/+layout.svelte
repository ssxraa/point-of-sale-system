<script>
  import Sidebar from '$lib/components/common/Sidebar.svelte';
  import { page } from '$app/stores';
  $: showSidebar = $page.url.pathname !== '/' ;
</script>

<div class="app-layout" >
  {#if showSidebar}
      <Sidebar />
  {/if}
  <main>
    <slot />
  </main>
</div>

<style>
main {
  margin-left: calc(67px + 2rem);
  margin-top: 1rem;
  margin-right: 1rem;
  font-family: 'goia regular';
}


</style>