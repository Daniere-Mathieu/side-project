<script lang="ts">
  import type { Project } from "src/project";
  import { parseTauriCommand } from "@src/utils/generics";
  import { onMount } from "svelte";
  import queryString from "query-string";
  import { navigate } from "svelte-routing/src/history";
  import { convertFileSrc } from "@tauri-apps/api/tauri";

  let id = Number(queryString.parse(location.search).id as string);
  let project: Project = {
    id,
    name: "",
    description: "",
    created_at: "",
    updated_at: "",
    logo: "",
  };

  async function getData() {
    try {
      if (!id) {
        navigate("/");
        return;
      }
      project = await parseTauriCommand<Project>("get_project", { id });
    } catch (error) {
      console.error(error);
    }
  }
  onMount(() => {
    getData();
  });
</script>

<div class="detail">
  <div class="detail-container">
    <div>
      <img src={convertFileSrc(project.logo)} alt={project.name} />
    </div>
    <div>
      <h1>{project.name}</h1>
      <p>{project.description}</p>
    </div>
  </div>
</div>
