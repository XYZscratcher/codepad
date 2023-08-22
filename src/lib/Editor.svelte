<script>
  export let file;
  import { onMount } from "svelte";
  //import { format } from "prettier";
  import { invoke } from "@tauri-apps/api/tauri";
  //import { argv } from "node:process";
  //import { getHighlighter } from "shikiji";

  let code = ""; /*,html = ""*/
  let editor, line_number,html="";
  let error = false,
    errorInfo = "";

  invoke("read_file", { path: file.path })
    .then((v) => {
      /*if (v === "Invalid data") {
      //TODO:
      errorInfo = "文件的内容不是有效的 UTF-8，无法打开文件。";
      error = true;
    } else {*/
      code = v;
      //}
    })
    .catch((e) => {
      error = true;
      if (e === "Invalid data") {
        errorInfo = "文件的内容不是有效的 UTF-8，无法打开文件。";
      }else{
        errorInfo="其他错误"
      }
    }); //TODO:
  onMount(() => {
    editor.focus();
    editor.addEventListener("scroll", () => {
      line_number.scrollTop = editor.scrollTop;
    });
  });
  
  /*(async () => {
    const shiki = await getHighlighter({
      themes: ["nord"],
      langs: ["javascript"],
    });

    // optionally, load themes and languages after creation
    await shiki.loadTheme("vitesse-light");
    await shiki.loadLanguage("css");
    const c = shiki.codeToHtml(code, {
      lang: "javascript",
      theme: "vitesse-light",
    });
    html=c.replace(/<pre .+?>/,"").replace(/<\/pre>/,"").replace(/<code>/,"")
  })();*/ //TODO:
  $: lines = html.replaceAll("<div><br></div>","<br>").replaceAll("</div>", "").replaceAll("<div>","<br>").split("<br>");
  console.log(file.size);
</script>

<div id="main" class="wrapper">
  {#if !error}
    <div id="line_number" bind:this={line_number}>
      {#each lines as _, i}
        <div class="line_number">{i + 1}</div>
      {/each}
    </div>

    <pre
      class="Editor"
      bind:this={editor}
      bind:innerText={code}
      bind:innerHTML={html}
      on:input={() => {
        //html=code
        invoke("save", { code, path: file.path });
        
        /*format(code, { semi: true, filepath: file.path }).then((c) => {
          code = c;
          console.log(c);
        });*/
      }}
      
      contenteditable
    />
  {:else}
    <h2>{errorInfo}</h2>
  {/if}
</div>

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
  .wrapper {
    display: flex;
  }
  #line_number {
    overflow-y: hidden;
    min-width: 3rem;
    margin-right: 1rem;
  }
  .Editor::-webkit-scrollbar {
    display: none;
  }
  /*:global(.Editor>div:has( br)){
    display: inline;
  }*/
</style>
