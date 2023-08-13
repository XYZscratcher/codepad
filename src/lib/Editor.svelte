<script>
  import { invoke } from "@tauri-apps/api/tauri";
  let code = "";
</script>

<textarea
  class="Editor"
  id="main"
  bind:value={code}
  on:input={() => {
    invoke("save", { code });
  }}
/>

<style>
  .Editor {
    display: block;

    font-family: "Maple Mono", "Fira Code", consolas, "Courier New", Courier,
      monospace;
    font-size: 1rem;

    outline: none;
    border: none;
  }
</style>
