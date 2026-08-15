<script>
    let { id, model, display_size, current_image, onclick } = $props();
</script>

<div class="display-frame">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="img-wrapper" {onclick} data-model={model}>
        {#if current_image}
            <img
                class="displayed-image"
                alt="current preview displayed on display"
                src="/api/images/{current_image}"
            />
        {/if}
        <img
            class="frame-overlay"
            alt="display model frame"
            src="/{model}.svg"
        />
    </div>
    <div class="display-info">
        <span class="id">#{id}</span>
        <span class="model">{model}</span>
        <span class="size">{display_size[0]}x{display_size[1]}</span>
    </div>
</div>

<style>
    .display-info {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 0.75rem;

        width: fit-content;
        margin-top: 1.5rem;
        margin-inline: auto;
        padding: 0.5rem 1.25rem;
        border-radius: 999px;

        background-color: var(--rpi-red-color);
        color: var(--ui-selection-white);

        font-size: 0.875rem;
        font-weight: 800;
        letter-spacing: 0.025em;
    }

    .img-wrapper {
        position: relative;
        width: 100%;
        aspect-ratio: 1 / 1.15;
        margin-inline: auto;
        cursor: pointer;
    }

    .frame-overlay {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: 2;
        pointer-events: none;
    }

    .displayed-image {
        position: absolute;
        width: 101%;
        aspect-ratio: 1 / 1;
        object-fit: cover;
        object-position: center;
    }

    .img-wrapper[data-model="GC9A01"] .displayed-image,
    .img-wrapper[data-model="ST7789"] .displayed-image {
        top: 0;
        left: 0;
    }

    .img-wrapper[data-model="ST7735"] .displayed-image {
        bottom: 0;
        left: 0;
    }

    .img-wrapper[data-model="GC9A01"] .displayed-image {
        clip-path: circle(46% at 50% 50%);
    }

    .img-wrapper[data-model="ST7789"] .displayed-image {
        clip-path: inset(2% 2% 2% 2% round 1.25rem);
    }

    .img-wrapper[data-model="ST7735"] .displayed-image {
        clip-path: inset(2% 2% 2% 2%);
    }
</style>
