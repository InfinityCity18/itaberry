<script>
    import displaySrc from "./assets/display.svg";
    import imageLogoSrc from "./assets/image.svg";
    let activeLeft = $state(true);
</script>

<div class="tab-switcher">
    <button class={["tab", { activeLeft }]} onclick={() => (activeLeft = true)}>
        <img class="display-logo" src={displaySrc} />
        <span class="label">Displays</span>
    </button>

    <button
        class={["tab", { activeLeft: !activeLeft }]}
        onclick={() => (activeLeft = false)}
    >
        <img class="image-logo" src={imageLogoSrc} />
        <span class="label">Images</span>
    </button>
</div>

<style>
    .display-logo {
        width: 2.1rem;
    }
    .image-logo {
        width: 2.1rem;
    }
    .tab-switcher {
        display: flex;
        align-items: center;
        background-color: var(--ui-grey-light);
        border-radius: 1rem;
        padding: 0.3rem;
        gap: 0.5rem;
        grid-column: 2;
    }

    .tab {
        display: flex;
        align-items: center;
        gap: 0.7rem;
        padding: 0.6rem 1.1rem;
        border: none;
        border-radius: 1rem;
        background: transparent;
        color: #2b2b2b;
        font-weight: 800;
        font-size: 1.1rem;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
    }

    /* --- Active Tab Styling --- */
    .tab.activeLeft {
        background-color: var(--ui-selection-white);
        color: var(--rpi-red-color);
    }
</style>
