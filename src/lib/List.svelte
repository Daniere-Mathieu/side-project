<script lang="ts">
  import { parseTauriCommand } from "/src/utils/generics";
  import type { Project } from "src/project";
  import { Link } from "svelte-routing";
  import Deleted from "./Deleted.svelte";
  import { onMount } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { Status } from "/src/project";
  import moment from "moment";
  import { sendNotification } from "@tauri-apps/api/notification";
  import StatusIcon from "./StatusIcon.svelte";

  let list: Project[] = [];

  async function getList() {
    try {
      list = await parseTauriCommand<Project[]>("get_projects");
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
    if (diff > 7)
      sendNotification({
        title: "Project",
        body: `Project ${item.name} need to work on it ,the last update was ${diff} days ago`,
      });
    return diff;
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
