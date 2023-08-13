<script>
  import Editor from "./lib/Editor.svelte";
  import Home from "./lib/Home.svelte";

  import "./styles.css";

  let code = "";
  let startEdit = false;
</script>

<main class="container">
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
    <Editor bind:value={code} on:input={() => {}} />
  {:else}
    <Home />
  {/if}
</main>

<style>
</style>
