<script>
    import { open } from "@tauri-apps/api/dialog";
    import { sep } from "@tauri-apps/api/path";
    import { invoke } from "@tauri-apps/api/tauri";
    import { Load } from "../store/load.js";
    import { notifications } from "../store/notifications.js";
    import { SuperStore } from "../store/superstore.js";
    import Toast from "../toast/Toast.svelte";
    
    let name;
    let query;
    let filePath;
    let browse_option;
    let yes = false;

    const fileName = async () => {
        try {
            filePath = await open({
                multiple: false,
                title: "Open",
                directory: yes ? true : false,
                filters: [{ name:"filter", extensions: [browse_option] }],
            });
            if (filePath) {
                name = filePath.substring(filePath.lastIndexOf(sep) + 1);
                // invoke("fileinput", { filename: filePath });
            }
        } catch (error) {
            console.error(error);
        }
    };

    const execute = () => {
        if (!filePath) {
            notifications.warning("Please browse the file", 5000);
            return;
        }
        if (!query) {
            notifications.warning("Please enter sql to execute", 5000);
            return;
        }
        let isLoad = {"load":true};
        Load.update(() => isLoad );
        invoke("execute_query", { query: query, file: filePath, filetype:browse_option })
            .then((res) => {
                SuperStore.update(() => res);
                res = null;
            })
            .catch((e) => {
                console.error(e);
                SuperStore.update(() => [{ error: e }]);
            });
    };

    const onKeyPress = (e) => {
        if ((e.ctrlKey || e.metaKey) && (e.keyCode == 13 || e.keyCode == 10)) {
            e.preventDefault();
            document.getElementById("execute").click();
        }
    };

    const reset = (e) => {
        name = "";
        filePath = "";
        SuperStore.update(() => []);
        let isLoad = {"load":false};
        Load.update(() => isLoad );
    }
</script>

<!-- editor section goes here -->
<section class="editor-section">
    <div class="toast">
        <Toast />
    </div>
    <div>
        <button id="file" on:click={fileName}>Browse : </button>
        {#if name}
            <span id="path">
                {name}
            </span>
            <span id="hint"> Hint: Table name is sample </span>
        {/if}
        <span id="filetype">
            <label for="filetype">Choose File Type</label>
            <select name="filetype" id="filetype" bind:value={browse_option}>
                <option value="parquet">Parquet</option>
                <option value="avro">Avro</option>
                <option value="csv">Csv</option>
                <option value="json">Json</option>
            </select>
        </span>
        <span id="resourcetype">
            File
            <label class="switch">
                <input type="checkbox" bind:checked={yes} on:change={reset}/>
                <span class="slider round" />
            </label>
                             Folder
        </span>
    </div>
    <div id="area">
        <textarea
            id="sqltext"
            placeholder="Select * from sample limit 10"
            bind:value={query}
            on:keypress={onKeyPress}
        />
        <div id="sqlcommand" class="sqlcommand">
            <button id="execute" on:click={execute(query)} type="submit"
                >run ►</button
            >
        </div>
    </div>
</section>

<style>
    #sqltext {
        width: -webkit-fill-available;
        min-height: 150px;
        resize: none;
        border-radius: 10px;
        /* padding: 0px; */
        /* display: block; */
    }
    #hint,
    #path,
    #filetype,
    #resourcetype,
    #file,
    #execute,
    #sqltext,
    #resourcetype,
    #sqltext::placeholder {
        font-family: "Roboto", sans-serif;
        font-size: medium;
    }
    #path,
    #file,
    #filetype,
    #resourcetype,
    #execute {
        background-color: #0a0a23;
        color: #fff;
        border: none;
        border-radius: 10px;
        padding: 7px 7px;
        width: fit-content;
    }
    #hint {
        background-color: white;
        color: black;
        border: none;
        border-radius: 10px;
        padding: 7px 7px;
        width: fit-content;
    }
    #file,
    #execute {
        cursor: pointer;
        z-index: 1;
        position: relative;
    }
    #file:active,
    #execute:active {
        top: 2px;
    }
    .toast {
        margin-left: auto;
        margin-right: auto;
    }
    #area {
        margin-left: 5px;
    }

    .switch {
        position: absolute;
        display: inline-block;
        width: 60px;
        height: 32px;
        margin: 3px 5px;
    }

    .switch input {
        opacity: 0;
        width: 0;
        height: 0;
    }

    .slider {
        position: absolute;
        cursor: pointer;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(180, 168, 59, 0.33);
        -webkit-transition: 0.4s;
        transition: 0.4s;
    }

    .slider:before {
        position: absolute;
        content: "";
        height: 20px;
        width: 20px;
        left: 4px;
        bottom: 6px;
        background-color: white;
        -webkit-transition: 0.4s;
        transition: 0.4s;
    }

    input:checked + .slider {
        background-color: grey;
    }

    input:focus + .slider {
        box-shadow: 0 0 1px grey;
    }

    input:checked + .slider:before {
        -webkit-transform: translateX(26px);
        -ms-transform: translateX(26px);
        transform: translateX(26px);
    }

    /* Rounded sliders */
    .slider.round {
        border-radius: 34px;
        bottom: 2px;
    }

    .slider.round:before {
        border-radius: 50%;
    }

    #resourcetype {
        color: white;
    }
</style>
