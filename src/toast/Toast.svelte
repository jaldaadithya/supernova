<script>
    import { flip } from "svelte/animate";
    import { fly } from "svelte/transition";
    import { notifications } from "../store/notifications.js";

    export let themes = {
        danger: "#E26D69",
        success: "#84C991",
        warning: "#074885",
        info: "#5bc0de",
        default: "#aaaaaa",
    };
</script>

<div class="notifications">
    {#each $notifications as notification (notification.id)}
        <div
            animate:flip
            class="toast"
            style="background: {themes[notification.type]};"
            transition:fly={{ x: -300 }}
        >
            <div class="content">{notification.message}</div>
            {#if notification.icon}<i class={notification.icon} />{/if}
        </div>
    {/each}
</div>

<style>
    .notifications {
        position: fixed;
        top: 70px;
        left: 0;
        right: 0;
        margin: 0 auto;
        padding: 0;
        z-index: 9999;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        pointer-events: none;
    }

    .toast {
        flex: 0 0 auto;
        margin-bottom: 10px;
        border-radius: 10px;
        color: black;
    }

    .content {
        padding: 10px;
        display: block;
        color: white;
        font-weight: 500;
        font-family: "Roboto", sans-serif;
    }
</style>
