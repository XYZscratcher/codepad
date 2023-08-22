<script>
  import { listen } from "@tauri-apps/api/event";
  //import {} from "@tauri-apps/plugin-fs"
  import Editor from "./lib/Editor.svelte";
  import Home from "./lib/Home.svelte";

  import "./styles.css";

  let startEdit = false;
  let fileInfo = {};
  
  listen("args", (e) => {
    const {payload}=e;
    console.log("args:",payload);
    console.log(e);
    startEdit = true;
    fileInfo={path: payload[0]}
  })
  listen("open_file", ({ payload }) => {
    console.log(payload);
    startEdit = true;
    fileInfo = payload;
  });
  
  
</script>

<div class="container">
  <nav>
    <ul>
      {#each ["文件", "编辑"] as item}
        <li>{item}</li>
      {/each}
      <li on:click={()=>location.reload()}>回到主页</li><!--仅为调试方便使用-->
    </ul>
  </nav>
  <div
    class="toolbar"
    style=" grid-column: 1 / 2;grid-row: 2 / 36;background:lightgreen"
  >
    Toolbar
  </div>
  {#if startEdit}
    <Editor file={fileInfo} />
  {:else}
    <Home />
  {/if}
</div>

<style>
  nav {
    grid-column: 1 / 24;
    grid-row: 1 / 2;
    background: lightblue;

    display: inline-flex;
    
    flex-direction: row;
    line-height: 0;
    font-size: 1rem;
  }
  nav > ul {
    margin-left: -1rem;
  }
  nav > ul > li {
    display: inline;
    flex: auto;
    margin-right: 1rem;
  }
</style>
