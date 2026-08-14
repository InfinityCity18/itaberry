<script>
    import ImageFrame from "./ImageFrame.svelte";
    import { blur } from "svelte/transition";
    import { uploaded_files } from "./shared.svelte.js";
    import { currentpage } from "./shared.svelte.js";

    /**
     * @type {String[]}
     */
    let images = $state([]);

    $effect(() => {
        fetch("/api/images")
            .then((res) => res.json())
            .then((data) => (images = data));
    });

    $effect(() => {
        if (uploaded_files.files.length > 0) {
            currentpage.displays = false; //change current page to images so the user can now see the uploaded images
            Array.from(uploaded_files.files).forEach((file) => {
                handleUpload(file);
            });
            uploaded_files.files = new DataTransfer().files;
        }
    });

    /**
     * @param {File} file
     */
    async function handleUpload(file) {
        const formData = new FormData();
        formData.append("file", file);
        try {
            await fetch(`/api/upload`, {
                method: "POST",
                body: formData,
            });
            images.push(file.name);
        } catch (err) {
            console.error("Upload failed:", err);
        }
    }

    /**
     * @param {string} filename
     */
    async function onDelete(filename) {
        try {
            const res = await fetch(`/api/delete/${filename}`, {
                method: "DELETE",
            });
            if (res.ok) {
                images = images.filter((img) => img !== filename);
            }
        } catch (err) {
            console.error("Failed to delete image:", err);
        }
    }
</script>

<div transition:blur class="gallery-section">
    <div class="gallery-wrapper">
        <div class="gallery">
            {#each images as image (image)}
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
