<script lang="ts">
  import { parseTauriCommand } from "/src/utils/generics";
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  export let id: number;
  async function deleteProject() {
    await parseTauriCommand("delete_project", { id });
    dispatch("deleted");
  }
</script>

<button on:click={deleteProject}>Delete</button>
