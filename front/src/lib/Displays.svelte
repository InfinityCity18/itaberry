<script>
    import DisplayFrame from "./DisplayFrame.svelte";
    import { blur } from "svelte/transition";
    import ImageSelectorModal from "./ImageSelectorModal.svelte";

    /**
     * @type {any[]}
     */
    let displays = $state([]);
    let activeDisplayId = $state(null);

    $effect(() => {
        fetch("/api/displays")
            .then((res) => res.json())
            .then((data) => (displays = data));
    });
</script>

<div transition:blur class="displays-section">
    {#each displays as display (display.id)}
        <div class="display-frame">
            <DisplayFrame
                {...display}
                onclick={() => (activeDisplayId = display.id)}
            ></DisplayFrame>
        </div>
    {/each}
</div>

{#if activeDisplayId !== null}
    <ImageSelectorModal
        displayId={activeDisplayId}
        onClose={() => (activeDisplayId = null)}
        onSuccess={(/** @type {String} */ newFilename) => {
            displays = displays.map((d) =>
                d.id === activeDisplayId
                    ? { ...d, current_image: newFilename }
                    : d,
            );
        }}
    />
{/if}

<style>
    .displays-section {
        --item-width: 20.75rem;

        --calculated-gap: calc((100% - (3 * var(--item-width))) / 4);

        --gap: max(1.5rem, var(--calculated-gap));

        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        background-color: var(--dark-background);
        padding-top: 3rem;
        padding-bottom: 3rem;

        gap: var(--gap);
    }

    .display-frame {
        width: var(--item-width);
        flex-shrink: 0;
    }
</style>
