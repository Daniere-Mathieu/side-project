<script lang="ts">
  import { Status } from "/src/project";
  import IoMdPause from "svelte-icons/io/IoMdPause.svelte";
  import IoMdCheckmark from "svelte-icons/io/IoMdCheckmark.svelte";
  import IoIosClose from "svelte-icons/io/IoIosClose.svelte";
  import IoIosPlay from "svelte-icons/io/IoIosPlay.svelte";
  import IoIosArchive from "svelte-icons/io/IoIosArchive.svelte";

  export let status: Status;
</script>

<div class="status">
  {#if status === Status.Active}
    <IoIosPlay title="active" />
  {:else if status === Status.Inactive}
    <IoMdPause title="inactive" />
  {:else if status === Status.Finished}
    <IoMdCheckmark title="finished" />
  {:else if status === Status.Deleted}
    <IoIosClose title="deleted" />
  {:else if status === Status.Archived}
    <IoIosArchive title="archived" />
  {/if}
</div>
