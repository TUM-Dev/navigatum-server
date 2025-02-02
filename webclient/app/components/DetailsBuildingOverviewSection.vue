<script setup lang="ts">
import { useToggle } from "@vueuse/core";
import type { components } from "~/api_types";
import { BuildingOffice2Icon, ChevronDownIcon, ChevronRightIcon, ChevronUpIcon } from "@heroicons/vue/24/outline";

type BuildingsOverviewResponse = components["schemas"]["BuildingsOverviewResponse"];

const props = defineProps<{
  readonly buildings?: BuildingsOverviewResponse | null;
}>();

const [buildingsExpanded, toggleBuildingsExpanded] = useToggle(false);
const { t } = useI18n({ useScope: "local" });
const runtimeConfig = useRuntimeConfig();
</script>

<template>
  <section v-if="props.buildings" class="px-5 print:!hidden">
    <h2 class="pb-3 text-lg font-semibold text-zinc-800">{{ t("title") }}</h2>
    <!--  <NuxtLinkLocale class="no-underline" to="#">Übersichtskarte <ArrowRightIcon class="w-4 h-4" /></NuxtLinkLocale> -->
    <div class="grid grid-cols-1 gap-3 text-zinc-600 md:grid-cols-2 xl:grid-cols-3">
      <template v-for="(b, i) in props.buildings.entries" :key="b.id">
        <NuxtLinkLocale
          v-if="i < props.buildings.n_visible || buildingsExpanded"
          :to="'/view/' + b.id"
          class="focusable flex flex-row items-center justify-between rounded-xs border border-solid border-zinc-200 p-3.5 !no-underline hover:bg-zinc-100"
          :aria-label="t('show_details_for', [b.name])"
        >
          <div class="flex flex-row items-center gap-3">
            <figure v-if="b.thumb" class="max-h-11 min-h-11 min-w-11">
              <NuxtImg
                width="64px"
                height="64px"
                class="aspect-square h-11 w-11 rounded-full"
                :alt="t('thumbnail_preview')"
                densities="x1 x2"
                :src="`${runtimeConfig.public.cdnURL}/cdn/thumb/${b.thumb}`"
              />
            </figure>
            <div v-else class="min-w-11 rounded-full bg-blue-500 p-2 text-white">
              <BuildingOffice2Icon class="mx-auto h-7 w-7" />
            </div>
            <div class="flex flex-col justify-evenly">
              <div class="line-clamp-2 text-balance">{{ b.name }}</div>
              <small class="text-zinc-600">{{ b.subtext }}</small>
            </div>
          </div>
          <ChevronRightIcon class="h-4 w-4" />
        </NuxtLinkLocale>
      </template>
    </div>
    <div v-if="props.buildings.n_visible < props.buildings.entries.length" class="mt-2">
      <Btn
        variant="linkButton"
        :aria-label="buildingsExpanded ? t('show_less_buildings') : t('show_more_buildings')"
        @click="toggleBuildingsExpanded()"
      >
        <template v-if="buildingsExpanded">
          <ChevronUpIcon class="mt-0.5 h-4 w-4" />
          {{ t("less") }}
        </template>
        <template v-else>
          <ChevronDownIcon class="mt-0.5 h-4 w-4" />
          {{ t("more") }}
        </template>
      </Btn>
    </div>
  </section>
</template>

<i18n lang="yaml">
de:
  default_thumbnail_preview: Standard-Thumbnail, da kein Thumbnail verfügbar ist
  less: weniger
  more: mehr
  show_less_buildings: weniger Gebäude anzeigen
  show_more_buildings: mehr Gebäude anzeigen
  thumbnail_preview: Thumbnail, das eine Vorschau des Gebäudes zeigt
  title: Gebäude / Gebiete
  show_details_for: Details für das Gebäude '{0}' anzeigen
en:
  default_thumbnail_preview: Default-thumbnail, as no thumbnail is available
  less: less
  more: more
  show_less_buildings: show less buildings
  show_more_buildings: show more buildings
  thumbnail_preview: Thumbnail, showing a preview of the building
  title: Buildings / Areas
  show_details_for: show the details for the building '{0}'
</i18n>
