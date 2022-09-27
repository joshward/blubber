<script lang="ts">
  import { Route } from "svelte-navigator";

  import FullLayout from "@/layout/Full.svelte";
  import RootPage from "@/pages/Home/Root.svelte";
  import SettingsPage from "@/pages/Home/Settings.svelte";
</script>

<FullLayout>
  <Route path="/" component={RootPage} />
  <Route path="settings" component={SettingsPage} />
</FullLayout>
