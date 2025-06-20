<script>
  import Sidebar from '$lib/components/common/Sidebar.svelte';
</script>

<div class="app-layout" >
  <Sidebar />
  <main>
    <slot />
  </main>
</div>

<style>
main {
  flex-grow: 1;
  margin-left: calc(67px + 2rem);
  margin-top: 1rem;
  margin-right: 1rem;
  font-family: 'goia regular';
}
</style>