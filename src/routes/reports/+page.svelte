<script lang="ts">
  import ReportsContent from '$lib/components/reports/ReportsContent.svelte';

  // No specific script logic needed here, it's all in ReportsContent for now!
</script>

<div class="reports-page-wrapper">
  <ReportsContent />
</div>

<style>
  /*
    This wrapper ensures the single reports component is correctly positioned
    and styled within the overall layout.
  */
  .reports-page-wrapper {
    padding: 1rem; /* Consistent padding around the main content area */
    display: flex;
    justify-content: center; /* Center the reports container horizontally */
    align-items: flex-start; /* Crucial to start content at the top for proper scrolling */
    min-height: calc(100vh - 1rem); /* Adjust as needed for your app's header/footer if any */
  }

  /* No other styles are needed here, as the ReportsContent component handles its own styling. */
</style>