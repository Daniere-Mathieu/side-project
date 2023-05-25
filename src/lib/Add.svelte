<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { parseTauriCommand } from "/src/utils/generics";

  import { Status, type Project } from "/src/project";

  let project: Project = {
    id: 0,
    name: "",
    description: "",
    created_at: "",
    updated_at: "",
    status: "",
    logo: "",
  };
  let options = Object.values(Status) as string[];

  async function save() {
    try {
      console.log(project);
      project.created_at = new Date().toISOString();
      project.updated_at = new Date().toISOString();
      let test: boolean = await parseTauriCommand<boolean>("add_project", {
        project,
      });
      console.log(test);
    } catch (error) {
      console.error(error);
      // TODO: Show error message
    }
  }
</script>

<div>
  <div class="flex">
    <input type="text" bind:value={project.name} placeholder="name" />
    <textarea bind:value={project.description} placeholder="description" />
    <select bind:value={project.status}>
      {#each options as option}
        <option value={option}>
          {option}
        </option>
      {/each}
    </select>
    <input type="text" bind:value={project.logo} placeholder="logo" />
  </div>
  <button on:click={save}>test</button>
</div>
