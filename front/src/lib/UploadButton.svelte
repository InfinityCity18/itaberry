<script>
    import uploadIcon from "./assets/upload.svg";
    import { uploaded_files } from "./shared.svelte.js";
</script>

<label class="upload-button">
    <input
        multiple
        bind:files={uploaded_files.files}
        accept=".webp, .png, .jpeg, .jpg, .gif"
        id="file-upload"
        type="file"
    />
    <img class="upload-icon" alt="upload icon" src={uploadIcon} />
    <span>Upload a photo or a gif</span>
</label>

<style>
    #file-upload {
        display: none;
    }

    .upload-icon {
        width: 1.2rem;
    }
    .upload-button {
        display: flex;
        justify-content: center;
        gap: 0.5rem;
        align-items: center;
        font-size: 1.1rem;
        border-radius: 1rem;
        height: 4rem;
        padding: 0 1rem;
        grid-column: 3;
        justify-self: end;
        border: none;
        background-color: var(--ui-selection-white);
        cursor: pointer;
        color: var(--rpi-red-color);
        font-weight: 800;
        border: dashed;
    }
    .upload-button:hover {
        background-color: var(--ui-grey-light);
    }
</style>
