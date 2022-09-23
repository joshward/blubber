<script lang="ts">
  import { Router, Route } from "svelte-navigator";

  import mainPage from "@/pages/Main.svelte";
  import settingsPage from "@/pages/Settings.svelte";
</script>

<main>
  <Router>
    <Route path="/" component={mainPage} />
    <Route path="settings" component={settingsPage} />
  </Router>
</main>
