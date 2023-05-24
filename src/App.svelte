<script lang="ts">
  import List from "./lib/List.svelte";
  import SideBar from "./lib/SideBar.svelte";
  import { Router, Link, Route } from "svelte-routing";
  import Add from "./lib/Add.svelte";
  import Details from "./lib/Details.svelte";
  let url = "/";
</script>

<main class="">
  <SideBar />
  <Router {url}>
    <nav>
      <Link to="/">Home</Link>
      <Link to="/add">add</Link>
    </nav>
    <div>
      <Route path="/add" component={Add} />
      <Route path="/"><List /></Route>
      <Route path="/project/:id" component={Details} />
    </div>
  </Router>
</main>

<style>
</style>
