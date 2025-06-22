<script lang="ts">
  import ReportsContent from '$lib/components/reports/ReportsContent.svelte';

</script>

<div class="reports-page-wrapper">
  <ReportsContent />
</div>

<style>

  .reports-page-wrapper {
    padding: 1rem; 
    display: flex;
    justify-content: center; 
    align-items: flex-start;
    min-height: calc(100vh - 1rem);
  }

</style>