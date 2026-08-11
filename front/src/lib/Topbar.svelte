<script>
    import TogglePage from "./TogglePage.svelte";
    import UploadButton from "./UploadButton.svelte";
    import logoSrc from "./assets/logo.svg";
</script>

<header class="topbar">
    <img src={logoSrc} />
    <TogglePage></TogglePage>
    <UploadButton></UploadButton>
</header>

<style>
    .topbar {
        grid-template-columns: 1fr auto 1fr;
        display: grid;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem 1.5rem;
        background-color: #1a1a1a;
        color: white;
    }
</style>
