<script>
import DefaultHeader from "../../components/DefaultHeader.svelte";
let result = $state(null);

async function getData() {
  const url = "http://localhost:3000";

  try {
    const response = await fetch(url);

    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }

    result = await response.json();
  } catch (error) {
    console.error(error.message);
  }
}

getData();
</script>

<DefaultHeader />

<h1>List</h1>
{#if result}
  {#each result as item}
    <h2>{item.name}</h2>
    <b>Stats</b>
    <ul>
      <li><b>Region:</b> {item.region}</li>
      <li><b>State:</b> {item.state}</li>
      <li><b>Coordinates:</b> {item.coordinates}</li>
    </ul>
    <p>
      {item.description}
    </p>
  {/each}
{:else}
  <p>
    Loading...
  </p>
{/if}
