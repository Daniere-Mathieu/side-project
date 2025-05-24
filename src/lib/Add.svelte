<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { parseTauriCommand } from "@src/utils/generics";
  import { open } from "@tauri-apps/api/dialog";

  import { Status, type Project } from "@src/project";
  import { navigate } from "svelte-routing/src/history";

  let errorMessage: string = "";
  let project: Project = {
    id: 0,
    name: "",
    description: "",
    created_at: "",
    updated_at: "",
    logo: "",
  };

  let options = Object.values(Status) as string[];
  project.status = Status[options[0] as keyof typeof Status];

  /**
   * @description this function is used to add a logo to the project
   * @returns {Promise<void>}
   */
  async function addLogo(): Promise<void> {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Image",
          extensions: ["png", "jpeg", "jpg", "svg", "webp"],
        },
      ],
    });
    if (selected && typeof selected === "string") {
      project.logo = selected;
    }
  }

  /**
   * @description this function is used to save the data to the database
   * @returns {Promise<void>}
   */
  async function saveData(): Promise<void> {
    try {
      if (!project.name || !project.description || !project.status) {
        errorMessage = "Please fill all the fields";
        throw new Error("Please fill all the fields");
      }
      project.created_at = new Date().toISOString();
      project.updated_at = new Date().toISOString();
      await parseTauriCommand<boolean>("add_project", {
        project,
      });

      navigate("/");
    } catch (error) {
      console.error(error);
      errorMessage = error.message;
    }
  }
</script>

<div class="add-container">
  <div class="add">
    {#if errorMessage}
      <p class="error">{errorMessage}</p>
    {/if}
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
