<script lang="ts">
  import type { Project } from "src/project";
  import { parseTauriCommand } from "/src/utils/generics";

  let id = 5;
  async function getData() {
    try {
      let temp = await parseTauriCommand<Project>("get_project", { id });
      console.log(temp);
    } catch (error) {
      console.error(error);
    }
  }
</script>

<h1>Details Page</h1>
<button on:click={getData}>Get Data</button>
<p>Project ID: {id}</p>
