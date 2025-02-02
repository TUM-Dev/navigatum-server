<script setup lang="ts">
import { IndoorControl, MapServerHandler } from "maplibre-gl-indoor";
import { AttributionControl, FullscreenControl, GeolocateControl, Map, Marker, NavigationControl } from "maplibre-gl";
import { webglSupport } from "~/composables/webglSupport";
import type { IndoorMapOptions } from "maplibre-gl-indoor";
import type { components } from "~/api_types";
import type { GeoJSONSource } from "maplibre-gl";

type LocationDetailsResponse = components["schemas"]["LocationDetailsResponse"];
type Coordinate = components["schemas"]["Coordinate"];

const props = defineProps<{
  coords: LocationDetailsResponse["coords"];
  type: LocationDetailsResponse["type"];
}>();
const map = ref<Map | undefined>(undefined);
const marker = ref<Marker | undefined>(undefined);
const afterLoaded = ref<() => void>(() => {});
const runtimeConfig = useRuntimeConfig();
const zoom = computed<number>(() => {
  if (props.type === "building") return 17;
  if (props.type === "room") return 18;
  return 16;
});

onMounted(async () => {
  if (!webglSupport) return;

  const doMapUpdate = async function () {
    // The map might or might not be initialized depending on the type
    // of navigation.
    if (document.getElementById("interactive-indoor-map")) {
      if (document.getElementById("interactive-indoor-map")?.classList.contains("maplibregl-map")) {
        marker.value?.remove();
      } else {
        map.value = await initMap("interactive-indoor-map");

        document.getElementById("interactive-indoor-map")?.classList.remove("loading");
      }
    }

    if (map.value !== undefined) {
      const _marker = new Marker({ element: createMarker() });
      _marker.setLngLat([props.coords.lon, props.coords.lat]);
      _marker.addTo(map.value as Map);
      marker.value = _marker;
    }
  };

  // The map element should be visible when initializing
  if (!document.querySelector("#interactive-indoor-map .maplibregl-canvas")) await nextTick(doMapUpdate);
  else await doMapUpdate();
});

function createMarker(hueRotation = 0): HTMLDivElement {
  const markerDiv = document.createElement("div");
  const markerIcon = document.createElement("span");
  markerIcon.style.filter = `hue-rotate(${hueRotation}deg)`;
  markerIcon.classList.add("marker");
  markerIcon.classList.add("marker-pin");
  markerDiv.appendChild(markerIcon);
  const markerShadow = document.createElement("span");
  markerShadow.classList.add("marker");
  markerShadow.classList.add("marker-shadow");
  markerDiv.appendChild(markerShadow);
  return markerDiv;
}

async function initMap(containerId: string): Promise<Map> {
  const map = new Map({
    container: containerId,
    // while having the hash in the url is nice, it is overridden on map load anyway => not much use
    hash: false,

    canvasContextAttributes: {
      // create the gl context with MSAA antialiasing, so custom layers are antialiasing.
      // slower, but prettier and therefore worth it for our use case
      antialias: true,

      // without this true, printing the webpage is not possible
      // with this true the performance is halfed though...
      // => we are deliberetely not supporing printing of this part of the webpage
      preserveDrawingBuffer: false,
    },

    style: "/navigatum-basemap.json",

    center: [11.670099, 48.266921],
    zoom: zoom.value,

    // done manually, to have more control over when it is extended
    attributionControl: false,
  });

  // Each source / style change causes the map to get
  // into "loading" state, so map.loaded() is not reliable
  // enough to know whether just the initial loading has
  // succeeded.
  map.on("load", () => {
    map.addControl(new NavigationControl({}), "top-left");

    // (Browser) Fullscreen is enabled only on mobile, on desktop the map
    // is maximized instead. This is determined once to select the correct
    // container to maximize, and then remains unchanged even if the browser
    // is resized (not relevant for users but for developers).
    const isMobile = window.matchMedia && window.matchMedia("only screen and (max-width: 480px)").matches;
    const fullscreenContainer = isMobile
      ? document.getElementById("interactive-indoor-map")
      : document.getElementById("interactive-indoor-map-container");
    const fullscreenCtl = new FullscreenControl({
      container: fullscreenContainer as HTMLElement,
    });
    // "Backup" the maplibregl default fullscreen handler
    const defaultOnClickFullscreen = fullscreenCtl._onClickFullscreen;
    fullscreenCtl._onClickFullscreen = () => {
      if (isMobile) defaultOnClickFullscreen();
      else {
        if (fullscreenCtl._container.classList.contains("maximize")) {
          fullscreenCtl._container.classList.remove("maximize");
          document.body.classList.remove("overflow-y-hidden");
          fullscreenCtl._fullscreenButton.classList.remove("maplibregl-ctrl-shrink");
        } else {
          fullscreenCtl._container.classList.add("maximize");
          fullscreenCtl._fullscreenButton.classList.add("maplibregl-ctrl-shrink");
          document.body.classList.add("overflow-y-hidden");
          window.scrollTo({ top: 0, behavior: "auto" });
        }

        fullscreenCtl._fullscreen = fullscreenCtl._container.classList.contains("maximize");
        fullscreenCtl._fullscreenButton.ariaLabel = fullscreenCtl._fullscreen ? "Exit fullscreen" : "Enter fullscreen";
        fullscreenCtl._fullscreenButton.title = fullscreenCtl._fullscreen ? "Exit fullscreen" : "Enter fullscreen";
        fullscreenCtl._map.resize();
      }
    };
    // There is a bug that the map doesn't update to the new size
    // when changing between fullscreen in the mobile version.
    if (isMobile) {
      const fullscreenObserver = new ResizeObserver(() => {
        fullscreenCtl._map.resize();
      });
      fullscreenObserver.observe(fullscreenCtl._container);
    }
    map.addControl(fullscreenCtl);

    const location = new GeolocateControl({
      positionOptions: {
        enableHighAccuracy: true,
      },
      trackUserLocation: true,
    });
    map.addControl(location);

    // The attributionControl is automatically open, which takes up a lot of
    // space on the small map display that we have. That's why we add it ourselves
    // and then toggle it.
    // It's only added after loading because if we add it directly on map initialization
    // for some reason it doesn't work.
    const attrib = new AttributionControl({ compact: true });
    map.addControl(attrib);
    attrib._toggleAttribution();

    map.addSource("route", {
      type: "geojson",
      data: {
        type: "Feature",
        properties: {},
        geometry: {
          type: "LineString",
          coordinates: [],
        },
      },
    });
    map.addLayer({
      id: "route",
      type: "line",
      source: "route",
      layout: {
        "line-join": "round",
        "line-cap": "round",
      },
      paint: {
        "line-color": "#e37222",
        "line-width": 7,
      },
    });
    map.addLayer({
      id: "route-symbol",
      type: "symbol",
      source: "route",
      layout: {
        "icon-image": "triangle",
        "icon-size": 0.25,
      },
      paint: {
        "icon-color": "#e37222",
      },
    });
    afterLoaded.value();
  });

  map.on("style.load", () => {
    map.setProjection({
      type: "globe", // Set projection to globe
    });
  });

  const indoorOptions = { showFeaturesWithEmptyLevel: false } as IndoorMapOptions;
  const mapServerHandler = MapServerHandler.manage(
    `${runtimeConfig.public.apiURL}/api/maps/indoor`,
    map,
    indoorOptions,
  );

  // Add the specific control
  mapServerHandler.map.addControl(new IndoorControl(), "bottom-left");

  return map;
}

function drawRoute(shapes: readonly Coordinate[], isAfterLoaded: boolean = false) {
  const src = map.value?.getSource("route") as GeoJSONSource | undefined;
  if (!src || (!isAfterLoaded && !map.value?.loaded())) {
    afterLoaded.value = () => drawRoute(shapes, true);
    return;
  }
  // cannot be undefined as returned from above if.. come on typescript
  src?.setData({
    type: "Feature",
    properties: {},
    geometry: {
      type: "LineString",
      coordinates: shapes.map(({ lat, lon }) => [lon, lat]),
    },
  });
  const latitudes = shapes.map(({ lat }) => lat);
  const longitudes = shapes.map(({ lon }) => lon);
  fitBounds([Math.min(...longitudes), Math.max(...longitudes)], [Math.min(...latitudes), Math.max(...latitudes)]);
}

function fitBounds(lon: [number, number], lat: [number, number]) {
  if (!map.value) {
    console.error("tried to fly to point but map has not loaded yet.. wtf??");
    return;
  }
  console.log("zooming to", { lat, lon });
  // below function zooms exactly to the values.
  // adding a bit of padding looks nicer
  const paddingLat = (lat[1] - lat[0]) * 0.1;
  const paddingLon = (lon[1] - lon[0]) * 0.1;
  map.value.fitBounds(
    [
      { lat: lat[0] - paddingLat, lng: lon[0] - paddingLon },
      { lat: lat[1] + paddingLat, lng: lon[1] + paddingLon },
    ],
    { maxZoom: 19 },
  );
}

defineExpose({ drawRoute, fitBounds });
</script>

<template>
  <div
    id="interactive-indoor-map-container"
    class="!h-full min-h-96 print:!hidden"
    :class="{
      'border border-zinc-300 bg-white dark:bg-black': webglSupport,
      'bg-red-300 text-red-950': !webglSupport,
    }"
  >
    <div v-if="webglSupport" id="interactive-indoor-map" class="relative !h-full min-h-96 !w-full" />
    <MapGLNotSupported v-else />
  </div>
</template>

<style>
@reference "../assets/main.css";
@import "maplibre-gl/dist/maplibre-gl.css";

/* --- Interactive map display --- */
#interactive-indoor-map-container {
  /* --- User location dot --- */

  .maplibregl-user-location-dot,
  .maplibregl-user-location-dot::before {
    @apply bg-blue-500;
  }

  > div {
    padding-bottom: 0;
  }

  &.maximize {
    position: absolute;
    top: 60px;
    left: 0;
    height: calc(100vh - 150px);
    z-index: 1000;
  }
}

.marker {
  position: absolute;
  pointer-events: none;
  padding: 0;

  &.marker-pin {
    background-image: url(~/assets/map/marker_pin.webp);
    width: 25px;
    height: 36px;
    top: -33px;
    left: -12px;
  }

  &.marker-shadow {
    background-image: url(~/assets/map/marker_pin-shadow.webp);
    width: 38px;
    height: 24px;
    top: -20px;
    left: -12px;
  }
}
</style>
