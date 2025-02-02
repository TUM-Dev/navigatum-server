<script setup lang="ts">
import type { BackgroundLayerSpecification, Coordinates, ImageSource } from "maplibre-gl";
import { AttributionControl, FullscreenControl, GeolocateControl, Map, Marker, NavigationControl } from "maplibre-gl";
import { FloorControl } from "~/composables/FloorControl";
import { webglSupport } from "~/composables/webglSupport";
import type { components } from "~/api_types";

const props = defineProps<{
  coords: LocationDetailsResponse["coords"];
  type: LocationDetailsResponse["type"];
  maps: LocationDetailsResponse["maps"];
  id: LocationDetailsResponse["id"];
  debugMode: boolean;
}>();
const map = ref<Map | undefined>(undefined);
const marker = ref<Marker | undefined>(undefined);
const floorControl = ref<FloorControl>(new FloorControl());
const runtimeConfig = useRuntimeConfig();
const zoom = computed<number>(() => {
  if (props.type === "building") return 17;
  if (props.type === "room") return 18;
  return 16;
});

const initialLoaded = ref(false);

type LocationDetailsResponse = components["schemas"]["LocationDetailsResponse"];

function loadInteractiveMap() {
  if (!webglSupport) return;

  const doMapUpdate = function () {
    // The map might or might not be initialized depending on the type
    // of navigation.
    if (document.getElementById("interactive-legacy-map")) {
      if (document.getElementById("interactive-legacy-map")?.classList.contains("maplibregl-map")) {
        marker.value?.remove();
      } else {
        map.value = initMap("interactive-legacy-map");

        document.getElementById("interactive-legacy-map")?.classList.remove("loading");
      }
    }
    if (map.value !== undefined) {
      const _marker = new Marker({ element: createMarker() });
      _marker.setLngLat([props.coords.lon, props.coords.lat]);
      // @ts-expect-error somehow this is too deep for typescript
      _marker.addTo(map.value as Map);
      marker.value = _marker;
    }

    const overlays = props.maps?.overlays;
    if (overlays) floorControl.value.updateFloors(overlays);
    else floorControl.value.resetFloors();

    map.value?.flyTo({
      center: [props.coords.lon, props.coords.lat],
      zoom: zoom.value,
      speed: 1,
      maxDuration: 2000,
    });
  };

  // The map element should be visible when initializing
  if (!document.querySelector("#interactive-legacy-map .maplibregl-canvas")) nextTick(doMapUpdate);
  else doMapUpdate();
}

function createMarker(hueRotation = 0) {
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

function initMap(containerId: string): Map {
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

    center: [11.5748, 48.14], // Approx Munich
    zoom: 11, // Zoomed out so that the whole city is visible

    attributionControl: false,
  });
  if (props.debugMode) {
    const debugMarker = new Marker({ draggable: true }).setLngLat([props.coords.lon, props.coords.lat]).addTo(map);

    debugMarker.on("dragend", () => {
      const lngLat = debugMarker.getLngLat();
      console.log(`debug marker "${props.id}": { lat: ${lngLat.lat}, lon: ${lngLat.lng} }`);
      navigator.clipboard.writeText(`"${props.id}": { lat: ${lngLat.lat}, lon: ${lngLat.lng} }`);
    });
  }

  map.on("style.load", () => {
    map.setProjection({
      type: "globe", // Set projection to globe
    });
  });

  // Each source / style change causes the map to get
  // into "loading" state, so map.loaded() is not reliable
  // enough to know whether just the initial loading has
  // succeeded.
  map.on("load", () => {
    initialLoaded.value = true;

    // controls
    map.addControl(new NavigationControl({}), "top-left");

    // (Browser) Fullscreen is enabled only on mobile, on desktop the map
    // is maximized instead. This is determined once to select the correct
    // container to maximize, and then remains unchanged even if the browser
    // is resized (not relevant for users but for developers).
    const isMobile = window.matchMedia && window.matchMedia("only screen and (max-width: 480px)").matches;
    const fullscreenContainer = isMobile
      ? document.getElementById("interactive-legacy-map")
      : document.getElementById("interactive-legacy-map-container");
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
  });

  interface FloorChangedEvent {
    file: string | null;
    coords: Coordinates | undefined;
  }

  floorControl.value.on("floor-changed", (args: FloorChangedEvent) => {
    const url = args.file ? `${runtimeConfig.public.cdnURL}/cdn/maps/overlays/${args.file}` : null;
    setOverlayImage(url, args.coords);
  });
  map.addControl(floorControl.value, "bottom-left");

  return map;
}

// Set the currently visible overlay image in the map,
// or hide it if imgUrl is null.
function setOverlayImage(imgUrl: string | null, coords: Coordinates | undefined) {
  // Even if the map is initialized, it could be that
  // it hasn't loaded yet, so we need to postpone adding
  // the overlay layer.
  // However, the official `loaded()` function is a problem
  // here, because the map is shortly in a "loading" state
  // when source / style is changed, even though the initial
  // loading is complete (and only the initial loading seems
  // to be required to do changes here)
  if (!initialLoaded.value) {
    map.value?.on("load", () => setOverlayImage(imgUrl, coords));
    return;
  }

  if (imgUrl === null) {
    // Hide overlay
    if (map.value?.getLayer("overlay")) map.value?.setLayoutProperty("overlay", "visibility", "none");
    if (map.value?.getLayer("overlay-bg")) map.value?.setLayoutProperty("overlay-bg", "visibility", "none");
  } else {
    const source = map.value?.getSource("overlay") as ImageSource | undefined;
    if (source === undefined) {
      if (coords !== undefined)
        map.value?.addSource("overlay", {
          type: "image",
          url: imgUrl,
          coordinates: coords,
        });
    } else
      source.updateImage({
        url: imgUrl,
        coordinates: coords,
      });

    const layer = map.value?.getLayer("overlay") as BackgroundLayerSpecification | undefined;
    if (!layer) {
      map.value?.addLayer({
        id: "overlay-bg",
        type: "background",
        paint: {
          "background-color": "#ffffff",
          "background-opacity": 0.6,
        },
      });
      map.value?.addLayer({
        id: "overlay",
        type: "raster",
        source: "overlay",
        paint: {
          "raster-fade-duration": 0,
        },
      });
    } else {
      map.value?.setLayoutProperty("overlay", "visibility", "visible");
      map.value?.setLayoutProperty("overlay-bg", "visibility", "visible");
    }
  }
}

// --- Loading components ---
onMounted(() => {
  nextTick(() => {
    // Even though 'mounted' is called there is no guarantee apparently,
    // that we can reference the map by ID in the DOM yet. For this reason we
    // try to poll now (Not the best solution probably)
    let timeoutInMs = 25;

    function pollMap() {
      const canLoadMap = document.getElementById("interactive-legacy-map") !== null;
      if (canLoadMap) {
        loadInteractiveMap();
        window.scrollTo({ top: 0, behavior: "auto" });
      } else {
        console.info(`'mounted' called, but page is not mounted yet. Retrying map-load in ${timeoutInMs}ms`);
        setTimeout(pollMap, timeoutInMs);
        timeoutInMs *= 1.5;
      }
    }

    pollMap();
  });
});
</script>

<template>
  <div
    id="interactive-legacy-map-container"
    class="aspect-[4 / 3] mb-2.5 print:!hidden"
    :class="{
      'border border-zinc-300 bg-white dark:bg-black': webglSupport,
      'bg-red-300 text-red-950': !webglSupport,
    }"
  >
    <div v-if="webglSupport" id="interactive-legacy-map" class="absolute !h-full !w-full" />
    <LazyMapGLNotSupported v-else />
  </div>
</template>

<style>
@reference "../assets/main.css";
@import "maplibre-gl/dist/maplibre-gl.css";

/* --- Interactive map display --- */
#interactive-legacy-map-container {
  /* --- User location dot --- */

  .maplibregl-user-location-dot,
  .maplibregl-user-location-dot::before {
    @apply bg-blue-500;
  }

  > div {
    padding-bottom: 75%; /* 4:3 aspect ratio */
  }

  &.maximize {
    position: absolute;
    top: 60px;
    left: 0;
    width: 100%;
    height: calc(100vh - 60px);
    z-index: 1000;

    > div {
      padding-bottom: 0;
      height: 100%;
    }
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

.maplibregl-ctrl-group.floor-ctrl {
  max-width: 100%;
  display: none;
  overflow: hidden;

  &.visible {
    display: block;
  }

  &.closed #floor-list {
    display: none !important;
  }

  & button {
    &.active {
      background: #ececec;
    }

    & .arrow {
      font-weight: normal;
      font-size: 0.3rem;
      line-height: 0.9rem;
      vertical-align: top;
    }
  }

  &.reduced > .vertical-oc,
  &.reduced > .horizontal-oc {
    display: none !important;
  }

  & > .vertical-oc,
  & > .horizontal-oc {
    font-weight: bold;
    background: #ececec;
  }

  &.closed {
    & > .vertical-oc,
    & > .horizontal-oc {
      background: #fff;
    }

    &:hover > .vertical-oc,
    &:hover > .horizontal-oc {
      background: #f2f2f2;
    }
  }

  /* vertical is default layout */

  & > .horizontal-oc {
    display: none;
  }

  &.horizontal {
    & > .horizontal-oc {
      display: inline-block;
    }

    & > .vertical-oc {
      display: none;
    }

    & #floor-list {
      display: inline-block;
      width: calc(100% - 29px);
    }

    & button {
      display: inline-block;
      border-top: 0;
      border-left: 1px solid #ddd;

      &.arrow {
        font-size: 0.4rem;
        vertical-align: bottom;
        line-height: 1.1rem;
      }

      & + button {
        border-top: 0;
      }
    }
  }
}
</style>
