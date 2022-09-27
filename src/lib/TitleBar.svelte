<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { showError } from "./utils";

  import IconButton from "./IconButton.svelte";
  import RemoveIcon from "./icons/Remove.svelte";
  import CloseIcon from "./icons/Close.svelte";

  // TODO - Tauri doesn't currently have a way to fetch a window's title https://github.com/tauri-apps/tauri/issues/5023
  const title = "Blubber";

  async function minimizeWindow() {
    try {
      await appWindow.minimize();
    } catch (err: unknown) {
      await showError(err);
    }
  }

  async function closeWindow() {
    try {
      await appWindow.close();
    } catch (err: unknown) {
      await showError(err);
    }
  }
</script>

<div
  data-tauri-drag-region
  class="
      p-2
      border-b border-slate-900 border-solid
      flex justify-between items-center
    "
>
  <div data-tauri-drag-region class="flex-1 select-none" />
  <div>
    <IconButton on:click={minimizeWindow}>
      <RemoveIcon className="h-5 w-5" />
    </IconButton>

    <IconButton on:click={closeWindow}>
      <CloseIcon className="h-5 w-5" />
    </IconButton>
  </div>
</div>
