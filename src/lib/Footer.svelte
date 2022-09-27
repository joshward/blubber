<script lang="ts">
  import { useNavigate } from "svelte-navigator";

  import IconButton from "./IconButton.svelte";
  import GearIcon from "./icons/Gear.svelte";

  const navigate = useNavigate();

  async function handleSettingsClick() {
    navigate("settings");
  }
</script>

<div
  class="
      p-2
      border-t border-slate-900 border-solid
      flex justify-between items-center
    "
>
  <div />

  <IconButton on:click={handleSettingsClick}>
    <GearIcon className="h-5 w-5" />
  </IconButton>
</div>
