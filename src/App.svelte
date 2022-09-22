<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
</script>

<main>
  {#await invoke("get_app_location")}
    <h1>Loading</h1>
  {:then value}
    <h1>{value}</h1>
  {/await}
</main>
