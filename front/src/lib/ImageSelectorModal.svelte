<script>
    import { fade, blur } from "svelte/transition";

    let { displayId, onClose, onSuccess } = $props();

    /**
     * @type {String[]}
     */
    let images = $state([]);

    $effect(() => {
        fetch("/api/images")
            .then((res) => res.json())
            .then((data) => (images = data));
    });

    /**
     * @param {string} filename
     */
    async function selectAndSendImage(filename) {
        try {
            await fetch(`/api/displays/${displayId}/${filename}`, {
                method: "POST",
            });
            onSuccess?.(filename);
            onClose();
        } catch (err) {
            console.error("Failed to assign image to display:", err);
        }
    }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="modal-backdrop"
    transition:fade={{ duration: 200 }}
    onclick={onClose}
>
    <div
        class="modal-card"
        transition:blur={{ amount: 10, duration: 200 }}
        onclick={(e) => e.stopPropagation()}
    >
        <div class="image-grid">
            {#each images as image (image)}
                <button
                    class="image-option"
                    onclick={() => selectAndSendImage(image)}
                >
                    <img src={`/api/images/${image}`} alt={image} />
                </button>
            {/each}
        </div>
    </div>
</div>

<style>
    .modal-backdrop {
        position: fixed;
        inset: 0;
        background-color: rgba(0, 0, 0, 0.75);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 100;
        backdrop-filter: blur(4px);
    }

    .modal-card {
        background-color: var(--dark-background, #1e1e1e);
        border: 1px solid rgba(255, 255, 255, 0.1);
        padding: 2rem;
        border-radius: 1.5rem;
        width: min(90%, 65rem);
        max-height: 80vh;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
    }

    .image-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr));
        gap: 1rem;
        overflow-y: auto;
        max-height: 50vh;
        padding: 0.5rem;
    }

    .image-option {
        background: rgba(255, 255, 255, 0.05);
        border: 2px solid transparent;
        border-radius: 0.75rem;
        padding: 0.5rem;
        cursor: pointer;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        align-items: center;
        transition: all 0.2s;
    }

    .image-option:hover {
        border-color: #ef4444;
        background: rgba(255, 255, 255, 0.1);
    }

    .image-option img {
        width: 100%;
        aspect-ratio: 1 / 1;
        object-fit: cover;
        border-radius: 0.5rem;
    }

    @media (max-width: 48rem) {
        .image-grid {
            grid-template-columns: repeat(auto-fill, minmax(7.5rem, 1fr));
            gap: 0.75rem;
        }

        .image-option {
            padding: 0.5rem;
            gap: 0.5rem;
        }
    }
</style>
