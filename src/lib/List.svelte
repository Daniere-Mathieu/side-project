<script lang="ts">
  import { parseTauriCommand } from "/src/utils/generics";
  import type { Project } from "src/project";
  import { Link, Router } from "svelte-routing";

  let list: Project[] = [];

  async function getList() {
    try {
      list = await parseTauriCommand<Project[]>("get_projects");
    } catch (error) {
      console.error(error);
    }
  }
</script>

<div>
  <div class="row">
    <button on:click={getList}>Get List</button>
  </div>
  <ul>
    {#each list as item}
      <li>
        <h2>{item.name}</h2>
        <p>{item.description}</p>
        <p>{item.status}</p>
        <img src={item.logo} alt="" />
        <Router>
          <Link to="/project/{item.id}">learn more</Link>
        </Router>
      </li>
    {/each}
  </ul>
</div>
<div />
