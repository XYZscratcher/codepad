<script>
    import { invoke } from "@tauri-apps/api";
    import { emit } from "@tauri-apps/api/event";
    import { open, save } from "@tauri-apps/plugin-dialog";
</script>

<div id="main">
    <h1>Hi, coder! Welcome to Codepad!</h1>
    <button
        on:click={() => {
            save({ title: "创建文件" }).then((p) => {
                if (p !== null) {
                    invoke("save", { code: "", path: p });
                    emit("open_file", { path: p, size: 0 });
                }
            });
        }}>Create a new file</button
    >
    <button
        on:click={() => {
            open().then((f) => {
                if (f !== null) {
                    emit("open_file", f);
                }
            });
        }}>Open a file</button
    >
</div>
