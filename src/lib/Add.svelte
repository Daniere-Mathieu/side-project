<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { parseTauriCommand } from "/src/utils/generics";

  import type { Project } from "src/project";

  let project: Project = {
    id: 0,
    name: "",
    description: "",
    created_at: "",
    updated_at: "",
    status: "",
    logo: "",
  };

  async function save() {
    try {
      project.created_at = new Date().toISOString();
      project.updated_at = new Date().toISOString();
      let test = await parseTauriCommand<boolean>("add_project", { project });
    } catch (error) {
      console.error(error);
      // TODO: Show error message
    }
  }
</script>

<div>
  <div class="flex">
    <input type="text" bind:value={project.name} placeholder="name" />
    <input
      type="text"
      bind:value={project.description}
      placeholder="description"
    />
    <input type="text" bind:value={project.status} placeholder="status" />
    <input type="text" bind:value={project.logo} placeholder="logo" />
  </div>
  <button on:click={save}>test</button>
</div>
