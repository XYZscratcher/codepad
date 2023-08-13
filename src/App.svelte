<script>
  import Editor from "./lib/Editor.svelte";
  import Home from "./lib/Home.svelte";

  import "./styles.css";

  
  let startEdit = true;
</script>

<div class="container">
  <nav style="grid-column: 1 / 24;grid-row: 1 / 2;background:lightblue">
    nav
  </nav>
  <div
    class="toolbar"
    style=" grid-column: 1 / 2;grid-row: 1 / 24;background:lightgreen"
  >
    Toolbar
  </div>
  {#if startEdit}
    <Editor />
  {:else}
    <Home />
  {/if}
</div>

<style>
</style>
