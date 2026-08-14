<script>
    import displaySrc from "./assets/display.svg";
    import imageLogoSrc from "./assets/image.svg";
    import { currentpage } from "./shared.svelte.js";
</script>

<div class="tab-switcher">
    <button
        class={["tab", { activeLeft: currentpage.displays }]}
        onclick={() => (currentpage.displays = true)}
    >
        <img class="display-logo" alt="display logo" src={displaySrc} />
        <span class="label">Displays</span>
    </button>

    <button
        class={["tab", { activeLeft: !currentpage.displays }]}
        onclick={() => (currentpage.displays = false)}
    >
        <img class="image-logo" alt="images logo" src={imageLogoSrc} />
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

    .tab.activeLeft {
        background-color: var(--ui-selection-white);
        color: var(--rpi-red-color);
    }
</style>
