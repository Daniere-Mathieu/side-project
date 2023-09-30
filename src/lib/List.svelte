<script lang="ts">
  import { parseTauriCommand, tauriNotify } from "/src/utils/generics";
  import type { Project } from "src/project";
  import { Link } from "svelte-routing";
  import Deleted from "./Deleted.svelte";
  import { afterUpdate, onMount } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { Status } from "/src/project";
  import moment from "moment";
  import StatusIcon from "./StatusIcon.svelte";
  import { hasInitialized } from "/src/store";

  let list: Project[] = [];

  async function getList() {
    try {
      list = await parseTauriCommand<Project[]>("get_projects");
      notify();
    } catch (error) {
      console.error(error);
    }
  }
  async function updateProject() {
    await getList();
  }

  onMount(getList);

  function computeTime(item: Project): number {
    if (item.status !== Status.Active && item.status !== Status.Inactive)
      return 0;
    let now = moment();
    let created = moment(item.created_at);
    let diff = now.diff(created, "days");
    return diff;
  }

  function notify() {
    if ($hasInitialized === false) {
      list.forEach((item) => {
        let itemDiff = computeTime(item);
        if (itemDiff > 7)
          tauriNotify({
            title: item.name,
            body: `Project ${item.name} need to work on it ,the last update was ${itemDiff} days ago`,
          });
      });
      $hasInitialized = true;
    }
  }
</script>

<div class="list-container">
  <ul class="list">
    {#each list as item}
      <li class="list-item">
        <div class="list-item-container">
          <img src={convertFileSrc(item.logo)} alt="" class="list-item-image" />
          <div class="list-item-content-container">
            <div class="list-item-title">
              <h2>title: <span class="yellow-text">{item.name}</span></h2>
            </div>
            <p>
              description: <span class="yellow-text">{item.description}</span>
            </p>
            <p class="list-item-status-container">
              Status: <span class="yellow-text">
                <StatusIcon status={item.status} />
              </span>
            </p>
            <p>
              created at: <span class="yellow-text"
                >{moment(item.created_at).format("D MMMM YYYY")}</span
              >
            </p>
            <p>
              last update: <span class="yellow-text"
                >{computeTime(item)} days</span
              >
            </p>
            <Link to="/project/{item.id}" class="yellow-text">learn more</Link>
            <Deleted on:deleted={updateProject} id={item.id} />
          </div>
        </div>
      </li>
    {/each}
  </ul>
</div>
<div />
