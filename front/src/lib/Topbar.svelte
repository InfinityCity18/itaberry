<script>
    import TogglePage from "./TogglePage.svelte";
    import UploadButton from "./UploadButton.svelte";
    import logoSrc from "./assets/logo.svg";
</script>

<header class="topbar">
    <img alt="logo" src={logoSrc} />
    <TogglePage></TogglePage>
    <UploadButton></UploadButton>
</header>

<style>
    .topbar {
        overflow: auto;
        gap: 1rem;
        display: flex;
        grid-template-columns: 1fr auto 1fr;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem 1.5rem;
        background-color: #1a1a1a;
        color: white;
    }
    @media (max-width: 500px) {
        .topbar {
            flex-direction: column;
        }
    }
</style>
