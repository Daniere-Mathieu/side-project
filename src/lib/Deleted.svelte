<script lang="ts">
  import { parseTauriCommand } from "/src/utils/generics";
  import { createEventDispatcher } from "svelte";
  import IoMdTrash from "svelte-icons/io/IoMdTrash.svelte";
  const dispatch = createEventDispatcher();

  export let id: number;
  async function deleteProject() {
    console.log("here");
    await parseTauriCommand("delete_project", { id });
    console.log("pass after delete");
    dispatch("deleted");
  }
  function handleKeydown(event: KeyboardEvent) {
    // Activate on Enter or Space keys
    if (event.key === "Enter" || event.key === " ") {
      deleteProject();
    }
  }
</script>

<div
  class="delete yellow-text"
  on:click={deleteProject}
  on:keydown={handleKeydown}
  tabindex="0"
  role="button"
  title="delete"
  aria-label="Delete project"
>
  <IoMdTrash />
</div>
