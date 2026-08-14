<script>
    import trashIcon from "./assets/trash-icon.svg";
    let { filename, onDelete } = $props();

    let isConfirming = $state(false);
    /**
     * @type {number | undefined}
     */
    let timeoutId = undefined;

    async function handleClick() {
        if (isConfirming) {
            clearTimeout(timeoutId);
            await onDelete(filename);
        } else {
            isConfirming = true;
            timeoutId = setTimeout(() => {
                isConfirming = false;
            }, 3500);
        }
    }
</script>

<div class="frame">
    <div class="img-wrap">
        <img
            alt="frame displaying file {filename}"
            src="/api/images/{filename}"
        />
    </div>
    <button
        class={["delete-button", { confirm: isConfirming }]}
        onclick={handleClick}
        aria-label="Delete image"
    >
        <img src={trashIcon} alt="Trash icon" draggable="false" />
    </button>
</div>

<style>
    .frame {
        width: 100%;
        border-radius: 1rem;
        overflow: hidden;
        background-color: var(--black-background);
        padding: 1.5vw;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .img-wrap {
        aspect-ratio: 1 / 1;
    }

    .frame img {
        display: block;
        width: 100%;
        height: 100%;
        object-fit: cover;
        object-position: center;
        border-radius: 0.5rem;
    }

    .delete-button {
        height: 3.65rem;
        width: 100%;
        background-color: var(--rpi-red-color);
        border: none;
        border-radius: 0.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        transition: background-color 0.2s ease;
    }

    .delete-button.confirm {
        background-color: var(--rpi-red-color-dark);
        animation: pulse 1.5s infinite;
    }

    .delete-button img {
        width: 5.5rem;
        height: 4.5rem;
        -webkit-touch-callout: none;
    }
</style>
