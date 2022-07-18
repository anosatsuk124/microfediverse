<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  const start_server = async () => await invoke("start_server");
  const shutdown_server = async () => await invoke("shutdown_server");
  const object_test = async () => await invoke("test");
</script>

<main>
  <h1>MicroFediverse</h1>

  <button on:click={start_server}> Start a server </button>

  <button on:click={shutdown_server}> Shutdown a server </button>

  <button on:click={object_test}> Test an ActivityPub object </button>
</main>

<style>
</style>
