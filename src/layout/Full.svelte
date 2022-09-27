<script lang="ts">
  import TitleBar from "@/lib/TitleBar.svelte";
</script>

<div class="flex flex-col h-screen bg-slate-800 text-gray-200">
  <TitleBar />

  <div class="flex-1">
    <slot />
  </div>
</div>
