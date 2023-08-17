<script>
  export let file;
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/tauri";
  let code = "";
  let editor,line_number;
  onMount(()=>{editor.addEventListener("scroll",()=>{
    line_number.scrollTop=editor.scrollTop;
  })})

  invoke("read_file", { path: file.path }).then((v) => {
    code = v;
  }); //TODO:
  $: lines=code.split("\n");
</script>

<div id="main" class="wrapper"><div id="line_number" bind:this={line_number}>
{#each lines as _,i}
  <div class="line_number">{i+1}</div>
{/each}
</div>
<pre
  class="Editor"
  bind:this={editor}
  bind:innerText={code}
  on:input={() => {
    invoke("save", { code,path:file.path });
  }}
  contenteditable
></pre></div>

<style>
  .Editor {
    white-space: pre;
    overflow-y: scroll;
    overflow-x: hidden;
    text-overflow: clip;
    margin: 0;
    text-align: left;

    font-family: "Maple Mono", "Fira Code", consolas, "Courier New", Courier,
      monospace;
    font-size: 1rem;

    outline: none;
    border: none;
  }
  .wrapper{
    display: flex;
  }
  #line_number{
    overflow-y:hidden;
    min-width: 3rem;
    margin-right: 1rem;
  }
  .Editor::-webkit-scrollbar{
    display: none;
  }
</style>
