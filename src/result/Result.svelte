<script>
    import { onDestroy } from "svelte";
    import Loader from "../loader/Loader.svelte";
    import { Load } from "../store/load.js";
    import { SuperStore } from "../store/superstore.js";

    let result = [];
    let columnNames = [];
    $:show_loader = false;
    let superstoreUnSubscribe = SuperStore.subscribe((data) => {
        if (data.length > 0) {
            columnNames = Object.keys(data[0]);
        }
        result = data;
        data = null;
        show_loader = false;
    });

    let loaderUnsubscribe = Load.subscribe(data => {
        result = [];
        show_loader = data.load
    });
    onDestroy(() => {
        superstoreUnSubscribe();
        loaderUnsubscribe();
    });
</script>

<!-- result section goes here -->
<section class="result-section">
    <div id="result">
    {#if result.length > 0}
        <table>
            <thead>
                <tr>
                    <th>No.</th>
                    {#each columnNames as col}
                        <th>{col}</th>
                    {/each}
                </tr>
            </thead>
            <tbody>
                {#each result as row, i}
                    <tr>
                        <td>{i+1}</td>
                        {#each columnNames as col}
                            <td>{row[col]}</td>
                        {/each}
                    </tr>
                {/each}
            </tbody>
        </table>
    {/if}
    {#if show_loader}
        <Loader/>
    {/if}
    </div>
</section>

<style>
    table {
        max-width: 100%;
        border: 0;
        /* white-space: nowrap; */
        border-collapse: collapse;
        /* border: 1px solid rgba(0, 0, 0, 0.12); */
        border-radius: 4px;
        background-color: white;
        margin-left: auto;
        margin-right: auto;
    }

    th {
        height: 56px;
        font-family: Roboto, sans-serif;
        -moz-osx-font-smoothing: grayscale;
        -webkit-font-smoothing: antialiased;
        font-size: 0.875rem;
        line-height: 1.375rem;
        font-weight: 500;
        letter-spacing: 0.00714em;
        text-decoration: inherit;
        text-transform: inherit;
        text-align: left;
        padding-right: 16px;
        padding-left: 16px;
        position: sticky;
        z-index: 2;
        top: 0;
        background-color: black;
        color: white;
    }

    tbody {
        font-family: Roboto, sans-serif;
        -moz-osx-font-smoothing: grayscale;
        -webkit-font-smoothing: antialiased;
        font-size: 0.875rem;
        line-height: 1.25rem;
        font-weight: 400;
        letter-spacing: 0.01786em;
        text-decoration: inherit;
        text-transform: inherit;
        overflow-y: scroll;
    }

    thead > tr {
        background-color: black;
    }

    tr {
        height: 52px;
        border-top-color: rgba(0, 0, 0, 0.12);
        border-top-width: 1px;
        border-top-style: solid;
    }
    tr:hover {
        background-color: rgba(0, 0, 0, 0.04);
    }

    td {
        padding-right: 16px;
        padding-left: 16px;
    }

    .result-section {
        margin-top: 10px;
        overflow-y: auto;
        height: 45vh;
        width: -webkit-fit-content;
        max-width: 95%;
        margin-left: auto;
        margin-right: auto;
        max-height: 500px;
    }
</style>
