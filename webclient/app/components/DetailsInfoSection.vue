<script setup lang="ts">
import type { components } from "~/api_types";

type LocationDetailsResponse = components["schemas"]["LocationDetailsResponse"];
type ImageInfoResponse = components["schemas"]["ImageInfoResponse"];

defineProps<{ data: LocationDetailsResponse }>();

const shownImage = defineModel<ImageInfoResponse>("shown_image");
const slideshowOpen = defineModel<boolean>("slideshow_open", {
  required: true,
});
const { t } = useI18n({ useScope: "local" });
const runtimeConfig = useRuntimeConfig();
</script>

<template>
  <!-- Information section (on mobile) -->
  <div v-if="data.props?.computed" class="col-5 col-sm-12 column mt-4 block lg:hidden">
    <h2 class="pb-3 text-lg font-semibold text-zinc-800">
      {{ t("info_title") }}
    </h2>
    <DetailsPropertyTable
      :id="data.id"
      :props="data.props"
      :name="data.name"
      :navigation-enabled="data.coords.accuracy !== 'building'"
    />
  </div>

  <!-- Informationen card (desktop) -->
  <!-- Some elements are currently duplicate, which is not optimal but should be okay
       as long as only little information is there -->
  <div class="hidden lg:block">
    <div class="max-w-sm rounded-lg border border-zinc-200 bg-white shadow-md shadow-zinc-500/5 dark:bg-zinc-100">
      <button
        v-if="data.imgs?.length && data.imgs[0]"
        type="button"
        class="focusable rounded-t-lg"
        @click="slideshowOpen = true"
      >
        <NuxtImg
          :alt="t('image_alt')"
          :src="`${runtimeConfig.public.cdnURL}/cdn/lg/${data.imgs[0].name}`"
          class="block h-auto w-full max-w-full rounded-t-lg bg-zinc-100"
          preload
          placeholder
          sizes="500px sm:200px md:300px md:400px"
          densities="x1 x2"
        />
      </button>
      <div class="px-5 py-3">
        <span class="sr-only">{{ t("info_title") }}</span>
        <DetailsPropertyTable
          v-if="data"
          :id="data.id"
          :props="data.props"
          :name="data.name"
          :navigation-enabled="data.coords.accuracy !== 'building'"
        />
        <div class="mt-3 grid gap-2">
          <Toast v-if="data.coords.accuracy === 'building'" level="warning" :msg="t('msg.inaccurate_only_building')" />
          <Toast
            v-if="data.type === 'room' && data.maps?.overlays?.default === null"
            level="warning"
            :msg="t('msg.no_floor_overlay')"
          />
          <Toast v-if="data.props?.comment" :msg="data.props.comment" />
        </div>
      </div>
    </div>
    <!-- <button class="btn btn-link">Mehr Infos</button> -->
  </div>
  <ClientOnly>
    <LazyDetailsImageSlideshowModal
      v-if="slideshow_open && !!data.imgs"
      v-model:shown_image="shownImage"
      v-model:slideshow_open="slideshowOpen"
      :imgs="data.imgs"
    />
  </ClientOnly>
</template>

<i18n lang="yaml">
de:
  image_alt: Header-Bild, zeigt das Gebäude
  info_title: Informationen
  msg:
    inaccurate_only_building: Die angezeigte Position zeigt nur die Position des Gebäude(teils). Die genaue Lage innerhalb des Gebäudes ist uns nicht bekannt.
    no_floor_overlay: Für den angezeigten Raum gibt es leider keine Indoor Karte.
en:
  image_alt: Header image, showing the building
  info_title: Information
  msg:
    inaccurate_only_building: The displayed position only shows the position of the building(part). The exact position within the building is not known to us.
    no_floor_overlay: There is unfortunately no indoor map for the displayed room.
</i18n>
