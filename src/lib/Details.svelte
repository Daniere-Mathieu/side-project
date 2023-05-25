<script lang="ts">
  import type { Project } from "src/project";
  import { parseTauriCommand } from "/src/utils/generics";
  import { onMount } from "svelte";

  let id = 1;
  async function getData() {
    try {
      let temp = await parseTauriCommand<Project>("get_project", { id });
      console.log(temp);
    } catch (error) {
      console.error(error);
    }
  }
  onMount(() => {
    getData();
  });
</script>

<h1>Details Page</h1>
<p>Project ID: {id}</p>
