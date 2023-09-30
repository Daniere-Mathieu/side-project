<script lang="ts">
  // this component is the main component of the app
  // it contains the sidebar and the routes
  // it also contains the router
  // the router is used to navigate between the routes
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
