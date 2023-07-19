<script lang="ts">
  import { parseTauriCommand } from "/src/utils/generics";
  import { createEventDispatcher } from "svelte";
  import IoMdTrash from "svelte-icons/io/IoMdTrash.svelte";
  const dispatch = createEventDispatcher();

  export let id: number;
  async function deleteProject() {
    await parseTauriCommand("delete_project", { id });
    dispatch("deleted");
  }
</script>

<div class="delete yellow-text">
  <IoMdTrash on:click={deleteProject} title="delete" />
</div>
