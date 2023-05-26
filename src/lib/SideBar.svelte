<script lang="ts">
  import { Link } from "svelte-routing";
  import IoIosAdd from "svelte-icons/io/IoIosAdd.svelte";
  import IoMdHome from "svelte-icons/io/IoMdHome.svelte";
  import type { SvelteComponent } from "svelte";

  let list: { path: string; icon: typeof SvelteComponent; title: string }[] = [
    { icon: IoMdHome, path: "/", title: "Home" },
    { icon: IoIosAdd, path: "/add", title: "Add" },
  ];
</script>

<nav class="sidebar">
  <ul class="sidebar-list">
    {#each list as item}
      <Link to={item.path} class="sidebar-list-item">
        <svelte:component this={item.icon} title={item.title} />
      </Link>
    {/each}
  </ul>
</nav>
