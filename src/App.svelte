<script lang="ts">
  import SideBar from "./lib/SideBar.svelte";
  import { Router, Route } from "svelte-routing";

  import Routes from "./lib/Routes.svelte";
  let url = "/";
</script>

<main class="main">
  <Router {url}>
    <div class="app-container">
      <SideBar />
      <div class="routes-container">
        <Routes />
      </div>
    </div>
  </Router>
</main>

<style>
</style>
