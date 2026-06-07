<script>
import DefaultHeader from "../../components/DefaultHeader.svelte";

import 'leaflet/dist/leaflet.css';
import { onMount } from 'svelte';

let result = $state(null);
let specificSite = $state(null);
let mapContainer;

onMount(async () => {
  await getData();

  const L = await import('leaflet');
  const map = L.map(mapContainer).setView([52.1, 4.5], 10);
  
  let coordinates = result.map((site) => {return [site.coordinates]})


  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; OpenStreetMap contributors'
  }).addTo(map);

  result.forEach(element => {
    if (!element.coordinates) return;

    if(element.coordinates.split(',').length === 2) {
      const [lat, lon] = element.coordinates.split(',').map(Number);
      const marker = L.marker([lat, lon]).addTo(map).bindPopup(`
        <b>${element.name}</b><br>
      `);

      marker.on('click', function () {
        specificSite = element;
      })
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
  <div bind:this={mapContainer} class="map"></div> 
{:else}
  <p>
    Loading...
  </p>
{/if}

{#if specificSite}
  <div class="siteDetails">
     <h1 class="siteTitle">{specificSite.name}</h1>
     {specificSite.description}
  </div>
{/if}

<style>
.siteTitle {
  margin-top: 0;
}

.siteDetails {
  padding: 15px;
  border: 1px solid black;
  border-radius: 10px;
  margin: 10px;
}

.map {
  height: 400px;
  width: 100%;
}
</style>
