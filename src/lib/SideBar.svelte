<script lang="ts">
  let list: string[] = [];
</script>

<nav>
  <ul>
    {#each list as item}
      <a href="/test">{item}</a>
    {/each}
  </ul>
</nav>
