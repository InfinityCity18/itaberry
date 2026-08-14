<script>
  import Displays from "./lib/Displays.svelte";
  import Images from "./lib/Images.svelte";
  import Topbar from "./lib/Topbar.svelte";
  import { currentpage, uploaded_files } from "./lib/shared.svelte.js";

  $effect(() => {
    if (uploaded_files.files.length > 0) {
    }
  });
</script>

<Topbar></Topbar>
{#if currentpage.displays}<Displays></Displays>{:else}<Images></Images>{/if}
