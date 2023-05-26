<script lang="ts">
  import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
  import { parseTauriCommand } from "/src/utils/generics";
  import { open } from "@tauri-apps/api/dialog";

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
  project.status = options[0];

  async function addLogo() {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Image",
          extensions: ["png", "jpeg", "jpg", "svg", "webp"],
        },
      ],
    });
    if (selected) {
      project.logo = selected;
    }
  }
  async function saveData() {
    try {
      if (!project.name || !project.description || !project.status) {
        throw new Error("Please fill all the fields");
        // ! notify user
      }
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

<div class="add-container">
  <div class="add">
    <input
      type="text"
      bind:value={project.name}
      placeholder="name"
      class="add-input"
    />
    <textarea
      bind:value={project.description}
      placeholder="description"
      class="add-input"
    />
    <select bind:value={project.status} class="add-input">
      {#each options as option}
        <option value={option}>
          {option}
        </option>
      {/each}
    </select>
    <button on:click={addLogo} class="add-button">Logo</button>
    {#if project.logo}
      <img src={convertFileSrc(project.logo)} alt="logo" class="add-logo" />
    {/if}

    <button on:click={saveData} class="add-button">Add</button>
  </div>
</div>
