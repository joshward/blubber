<script lang="ts">
  import { Router, Route } from "svelte-navigator";

  import HomePage from "@/pages/Home.svelte";
</script>

<main>
  <Router>
    <Route path="home/*" component={HomePage} />
    <Route let:location>ERROR: NO SUCH PAGE {location.pathname}</Route>
  </Router>
</main>
