<script setup lang="ts">
import type { components } from "~/api_types";
import SearchResultItemLink from "~/components/SearchResultItemLink.vue";

type SearchResponse = components["schemas"]["SearchResponse"];

defineProps<{
  data: SearchResponse;
  queryLimitBuildings: number;
  queryLimitRooms: number;
}>();
const { t } = useI18n({ useScope: "local" });
const route = useRoute();
</script>

<template>
  <div v-for="s in data.sections" :key="s.facet">
    <section class="flex flex-col gap-2">
      <h2 class="text-md font-semibold text-zinc-500">{{ t(`sections.${s.facet}`) }}</h2>
      <ul v-for="(e, i) in s.entries" :key="e.id" class="flex flex-col gap-3">
        <SearchResultItemLink v-if="i < s.n_visible" :highlighted="false" :item="e" />
      </ul>
      <p v-if="s.estimatedTotalHits > 10" class="text-sm text-zinc-500">
        {{ t("approx_results", s.estimatedTotalHits) }}
        <NuxtLinkLocale
          :to="
            s.facet === 'rooms'
              ? `/search?q=${route.query.q}&limit_buildings=${queryLimitBuildings}&limit_rooms=${queryLimitRooms + 50}`
              : `/search?q=${route.query.q}&limit_buildings=${queryLimitBuildings + 20}&limit_rooms=${queryLimitRooms}`
          "
          class="focusable rounded-xs text-blue-500 visited:text-blue-500 hover:text-blue-600 hover:underline"
          >{{ t("view_more") }}
        </NuxtLinkLocale>
      </p>
      <p v-else class="text-sm text-zinc-500">
        {{ t("results", s.estimatedTotalHits) }}
      </p>
    </section>
  </div>
</template>

<i18n lang="yaml">
de:
  sections:
    sites_buildings: Gebäude / Standorte
    rooms: Räume
  view_more: mehr anzeigen
  approx_results: ca. {count} Ergebnisse, bitte grenze die Suche weiter ein
  results: 1 Ergebnis | {count} Ergebnisse
en:
  sections:
    sites_buildings: Buildings / Sites
    rooms: Rooms
  view_more: view more
  approx_results: approx. {count} results, please narrow the search further
  results: 1 result | {count} results
</i18n>
