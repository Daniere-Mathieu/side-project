<script lang="ts">
  import { parseTauriCommand } from "/src/utils/generics";
  import { createEventDispatcher } from "svelte";
  import IoMdTrash from "svelte-icons/io/IoMdTrash.svelte";
  const dispatch = createEventDispatcher();

  export let id: number;
  /**
   * @description this function is used to delete a project
   * @returns {Promise<void>}
   */
  async function deleteProject() {
    await parseTauriCommand("delete_project", { id });
    dispatch("deleted");
  }
  /**
   * @description this function is used to handle the keydown event for accesibility
   * @param {KeyboardEvent} event
   * @returns {void}
   */
  function handleKeydown(event: KeyboardEvent) {
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
