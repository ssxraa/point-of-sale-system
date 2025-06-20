<script lang="ts">
  import SettingsContent from '$lib/components/settings/SettingsContent.svelte';
</script>

<div class="settings-page-wrapper">
  <SettingsContent />
</div>

<style>
  .settings-page-wrapper {
    padding: 0;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    overflow: hidden; 
  }

  /* No other styles are needed here, as the SettingsContent component handles its own styling. */
</style>