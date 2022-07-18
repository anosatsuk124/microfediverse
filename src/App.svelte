<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  const start_server = async () => await invoke("start_server");
  const shutdown_server = async () => await invoke("shutdown_server");
</script>

<main>
  <h1>Vite + Svelte</h1>

  <button on:click={start_server}> Start a server </button>

  <button on:click={shutdown_server}> Shutdown a server </button>

  <p>
    Check out <a href="https://github.com/sveltejs/kit#readme" target="_blank"
      >SvelteKit</a
    >, the official Svelte app framework powered by Vite!
  </p>

  <p class="read-the-docs">Click on the Vite and Svelte logos to learn more</p>
</main>

<style>
  .read-the-docs {
    color: #888;
  }
</style>
