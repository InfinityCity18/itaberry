<script>
  import Displays from "./lib/Displays.svelte";
  import Images from "./lib/Images.svelte";
  import Topbar from "./lib/Topbar.svelte";
  import {
    currentpage,
    uploaded_files,
    images_filenames,
  } from "./lib/shared.svelte.js";

  let images = $state();

  $effect(() => {
    if (uploaded_files.files.length > 0) {
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
      // @ts-ignore
      images_filenames.filenames.push(file.name);
    } catch (err) {
      console.error("Upload failed:", err);
    }
  }
</script>

<Topbar></Topbar>
<div>
  {#if currentpage.displays}<Displays></Displays>{:else}<Images></Images>{/if}
</div>
