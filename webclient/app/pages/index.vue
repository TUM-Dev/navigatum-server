<script setup lang="ts">
import { MapPinIcon } from "@heroicons/vue/24/outline";
import { ArrowRightIcon, ChevronDownIcon, ChevronRightIcon, ChevronUpIcon } from "@heroicons/vue/24/solid";
import type { components } from "~/api_types";

/**
 * @description This is a list of all sites, that are available in the system.
 * It is sorted by the number of rooms in the site, descending.
 * The first entry is the site with the most importance
 */
type SitesOverview = components["schemas"]["RoomsOverviewUsageChildResponse"] & {
  /**
   * Format: int64
   * @description A recommendation how many of the entries should be displayed by default.
   * The number is usually from 0-5.
   * More results might be displayed when clicking "expand".
   * If this field is not present, then all entries are displayed.
   *
   * @example 6
   */
  readonly n_visible: number;
  /**
   * @description A select list of buildings, that are in this site.
   * Derived from the areatree.
   */
  readonly children: readonly components["schemas"]["RoomsOverviewUsageChildResponse"][];
};

const { t } = useI18n({ useScope: "local" });
const localePath = useLocalePath();
const sites_overview: readonly SitesOverview[] = [
  {
    children: [
      { id: "mi", name: "Mathematik / Informatik" },
      { id: "mw", name: "Maschinenwesen" },
      { id: "physik", name: "Physik" },
      { id: "chemie", name: "Chemie" },
      { id: "garching-interims", name: "Interimshörsäle" },
      { id: "5532", name: "StudiTUM Garching" },
    ],
    id: "garching",
    n_visible: 4,
    name: "Garching Forschungszentrum",
  },
  {
    children: [
      { id: "zentralgelaende", name: "Zentralgelände" },
      { id: "nordgelaende", name: "Nordgelände" },
      { id: "suedgelaende", name: "Südgelände" },
      { id: "suedwestgelaende", name: "Südwestgelände" },
      { id: "0201", name: "StudiTUM Innenstadt" },
      { id: "2910", name: "RiWa 1 (HfP, Governance)" },
    ],
    id: "stammgelaende",
    n_visible: 3,
    name: "Stammgelände",
  },
  {
    children: [
      { id: "wzw-berg", name: "Gebiet 4100 Berg" },
      { id: "wzw-mitte", name: "Gebiet 4200 Mitte" },
      { id: "wzw-nord", name: "Gebiet 4300 Nord" },
      { id: "duernast", name: "Dürnast (Versuchsstation)" },
      { id: "roggenstein", name: "Roggenstein (Versuchsstation)" },
      { id: "thalhausen", name: "Thalhausen (Versuchsstation)" },
      { id: "veitshof", name: "Veitshof (Stallungen)" },
      { id: "viehhausen", name: "Viehhausen (Versuchsstation)" },
    ],
    id: "wzw",
    n_visible: 3,
    name: "Weihenstephan (Freising)",
  },
  {
    children: [
      { id: "mri", name: "MRI Klinikum rechts der Isar" },
      { id: "olympiapark", name: "Campus im Olympiapark" },
      { id: "cs", name: "Campus Straubing" },
      { id: "garching-hochbrueck", name: "Garching Hochbrück" },
      { id: "taufkirchen-ottobrunn", name: "Taufkirchen / Ottobrunn" },
    ],
    id: "others",
    n_visible: 5,
    name: t("sites_overview.others"),
  },
];

const title = computed(() => t("sites") + " - NavigaTUM");
useSeoMeta({
  title: title,
  ogTitle: title,
  description: t("description"),
  ogDescription: t("description"),
  ogImage: "https://nav.tum.de/navigatum-card.png",
  twitterCard: "summary",
});

const openPanels = ref<(boolean | undefined)[]>([]);
</script>

<template>
  <div class="flex flex-col justify-between gap-3 pt-8">
    <h1 class="!text-lg font-semibold text-zinc-600">{{ t("sites") }}</h1>
    <!-- <NuxtLink :to="localePath('#')" class="flex flex-row"><MapPinIcon class="h-4 w-4" /> {{ t("overview_map") }}</NuxtLink> -->
  </div>
  <div class="mt-5">
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      <div
        v-for="(site, siteIndex) in sites_overview"
        :key="site.id"
        class="flex flex-col gap-4 rounded-lg border-2 border-zinc-200 p-5"
      >
        <div>
          <NuxtLink
            v-if="site.id !== 'others'"
            :to="localePath('/view/' + site.id)"
            :aria-label="t('show_details_for_campus', [site.name])"
            class="focusable flex grow-0 flex-row justify-between rounded-sm text-zinc-700 !no-underline hover:text-blue-500"
          >
            <h2 class="text-md font-semibold">{{ site.name }}</h2>
            <ArrowRightIcon v-if="site.id" class="my-auto hidden h-6 w-6 md:block" />
          </NuxtLink>
          <h2 v-else class="text-md font-semibold text-zinc-700">{{ site.name }}</h2>
        </div>
        <div class="flex flex-col gap-3">
          <NuxtLink
            v-for="c in site.children.slice(0, openPanels[siteIndex] ? site.children.length : site.n_visible)"
            :key="c.id"
            :to="localePath('/view/' + c.id)"
            :aria-label="t('show_details_for_building', [c.name])"
            class="focusable flex flex-row justify-between rounded-sm text-blue-600 !no-underline hover:text-blue-500"
          >
            <div class="flex flex-row gap-2">
              <MapPinIcon class="my-auto h-4 w-5" />
              <span>{{ c.name }}</span>
            </div>
            <ChevronRightIcon class="my-auto hidden h-4 w-4 sm:block" />
          </NuxtLink>
          <div v-if="site.children.length > site.n_visible" class="mx-auto">
            <Btn
              v-if="openPanels[siteIndex]"
              variant="linkButton"
              :aria-label="t('less_aria')"
              @click="() => (openPanels[siteIndex] = false)"
            >
              <ChevronUpIcon class="h-4 w-4" />
              {{ t("less") }}
            </Btn>
            <Btn v-else variant="linkButton" :aria-label="t('more_aria')" @click="() => (openPanels[siteIndex] = true)">
              <ChevronDownIcon class="my-auto h-4 w-4" />
              {{ t("more") }}
            </Btn>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<i18n lang="yaml">
de:
  less: weniger
  less_aria: weniger Gebäude anzeigen
  more: mehr
  more_aria: mehr Gebäude anzeigen
  overview_map: Übersichtskarte
  sites: Standorte
  show_details_for_campus: show the details for the campus '{0}'
  show_details_for_building: show the details for the building '{0}'
  description: Finde Räume, Gebäude und andere Orte an der TUM mit Exzellenz. Eine moderne Alternative zum RoomFinder, entwickelt von Studierenden.
  sites_overview:
    others: Sonstige
en:
  less: less
  less_aria: show more buildings
  more: more
  more_aria: show more buildings
  overview_map: Overview Map
  sites: Sites
  show_details_for_campus: show the details for the campus '{0}'
  show_details_for_building: show the details for the building '{0}'
  description: Find rooms, buildings and other places at TUM with excellence. A modern alternative to RoomFinder, developed by students.
  sites_overview:
    others: Others
</i18n>
