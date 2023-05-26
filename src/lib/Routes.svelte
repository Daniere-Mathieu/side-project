<script lang="ts">
  import { Route } from "svelte-routing";
  import Add from "./Add.svelte";
  import Details from "./Details.svelte";
  import List from "./List.svelte";
</script>

<Route path="/add" component={Add} />

<Route path="/"><List /></Route>

<Route path="/project/:id" component={Details} />
