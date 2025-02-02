<script setup lang="ts">
import type { components } from "~/api_types";

type ImageInfoResponse = components["schemas"]["ImageInfoResponse"];
type CoordinateResponse = components["schemas"]["CoordinateResponse"];
type SourcesResponse = components["schemas"]["SourcesResponse"];

defineProps<{
  coords: CoordinateResponse;
  sources: SourcesResponse;
  image?: ImageInfoResponse;
}>();

const { t } = useI18n({ useScope: "local" });
</script>

<template>
  <section class="px-5">
    <h2 class="text-md font-semibold text-zinc-800">{{ t("title") }}</h2>
    <div class="text-sm text-zinc-600">
      <p>
        {{ t("base.title") }}:
        <span v-for="(e, i) in sources.base" :key="e.name">
          <NuxtLink v-if="e.url" :to="e.url" external>{{ e.name }}</NuxtLink>
          <template v-else>{{ e.name }}</template>
          <template v-if="i < sources.base.length - 1"> • </template>
        </span>
        <span v-if="sources.patched">
          <br />
          ({{ t("base.patched") }})
        </span>
      </p>
      <p v-if="image">
        {{ t("header_img") }}:
        <span>{{ image.author.text }}</span>
        <span v-if="image.source">
          •
          <NuxtLink v-if="image.source.url" :to="image.source.url" target="_blank" external>
            {{ image.source.text }}
          </NuxtLink>
          <template v-else>{{ image.source.text }}</template>
        </span>
        <span v-if="image.license">
          •
          <NuxtLink v-if="image.license.url" :to="image.license.url" target="_blank" external>
            {{ image.license.text }}
          </NuxtLink>
          <template v-else>{{ image.license.text }}</template>
        </span>
      </p>
      <p>
        {{ t("coords.title") }}:
        <span v-if="coords.source === 'navigatum'"> {{ t("coords.navigatum") }}</span>
        <span v-if="coords.source === 'roomfinder'">
          {{ t("coords.roomfinder") }}
        </span>
        <span v-if="coords.source === 'inferred'">
          {{ t("coords.inferred") }}
        </span>
      </p>
    </div>
  </section>
</template>

<i18n lang="yaml">
de:
  base:
    patched: Bei diesem Eintrag wurden automatische Korrekturen zu externen Daten angewandt
    title: Basisdaten
  coords:
    inferred: Automatisch berechnet aus den zugehörigen Räumen oder Gebäuden
    navigatum: NavigaTUM Mitwirkende
    roomfinder: Roomfinder
    title: Koordinaten
  header_img: Bild
  title: Quellen
en:
  base:
    patched: For this entry automatic patches were applied to external data
    title: Base data
  coords:
    inferred: Automatically computed based on the associated rooms or buildings
    navigatum: NavigaTUM Contributors
    roomfinder: Roomfinder
    title: Coordinates
  header_img: Image
  title: Sources
</i18n>
