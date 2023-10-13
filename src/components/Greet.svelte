<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let names = {
    firstname: "",
    lastname: "Teddy",
  };
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { names });
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input
      id="greet-input"
      placeholder="Enter a name..."
      bind:value={names.firstname}
    />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>
