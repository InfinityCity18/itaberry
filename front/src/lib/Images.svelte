<script>
    import ImageFrame from "./ImageFrame.svelte";
    import { blur } from "svelte/transition";
    import { uploaded_files } from "./shared.svelte.js";
    import { images_filenames } from "./shared.svelte.js";

    $effect(() => {
        fetch("/api/images")
            .then((res) => res.json())
            .then((data) => (images_filenames.filenames = data));
    });

    /**
     * @param {string} filename
     */
    async function onDelete(filename) {
        try {
            const res = await fetch(`/api/delete/${filename}`, {
                method: "DELETE",
            });
            if (res.ok) {
                images_filenames.filenames = images_filenames.filenames.filter(
                    (img) => img !== filename,
                );
            }
        } catch (err) {
            console.error("Failed to delete image:", err);
        }
    }
</script>

<div transition:blur class="gallery-section">
    <div class="gallery-wrapper">
        <div class="gallery">
            {#each images_filenames.filenames as image (image)}
                <ImageFrame filename={image} {onDelete} />
            {/each}
        </div>
    </div>
</div>

<style>
    .gallery-section {
        background-color: var(--dark-background);
    }

    .gallery-wrapper {
        width: 100vw;
        max-width: 100rem;
        margin-inline: auto;
        padding: 2rem;
        box-sizing: border-box;
    }

    .gallery {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 2rem;
    }

    @media (min-width: 48em) {
        .gallery {
            grid-template-columns: repeat(3, 1fr);
        }
    }

    @media (min-width: 64em) {
        .gallery {
            grid-template-columns: repeat(4, 1fr);
        }
    }
</style>
