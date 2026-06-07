<script>
import DefaultHeader from "../../components/DefaultHeader.svelte";

import 'leaflet/dist/leaflet.css';
import { onMount } from 'svelte';

let result = $state(null);
let mapContainer;

onMount(async () => {
  await getData();

  const L = await import('leaflet');
  const map = L.map(mapContainer).setView([52.1, 4.5], 10);
  
  let coordinates = result.map((site) => {return [site.coordinates]})


  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; OpenStreetMap contributors'
  }).addTo(map);

  coordinates.forEach(element => {
    if(element[0].split(',').length === 2) {
      const [lat, lon] = element[0].split(',').map(Number);
      L.marker([lat, lon]).addTo(map);
    }
  });


  return () => map.remove();
});

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

<h1>Map</h1>
{#if result}
  <p>Map here</p>
{:else}
  <p>
    Loading...
  </p>
{/if}
<div bind:this={mapContainer} class="map"></div> 
<style>
.map {
  height: 400px;
  width: 100%;
}
</style>
