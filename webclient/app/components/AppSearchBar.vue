<script setup lang="ts">
import { MagnifyingGlassIcon } from "@heroicons/vue/24/solid";
import type { components } from "~/api_types";
import SearchResultItemLink from "~/components/SearchResultItemLink.vue";

type SearchResponse = components["schemas"]["SearchResponse"];

const searchBarFocused = defineModel<boolean>("searchBarFocused", {
  required: true,
});
const { t, locale } = useI18n({ useScope: "local" });
const localePath = useLocalePath();
const route = useRoute();
const keep_focus = ref(false);
const query = ref(Array.isArray(route.query.q) ? (route.query.q[0] ?? "") : (route.query.q ?? ""));
const highlighted = ref<number | undefined>(undefined);
const sites_buildings_expanded = ref<boolean>(false);

const visibleElements = computed<string[]>(() => {
  if (!data.value) return [];

  const visible: string[] = [];
  data.value.sections.forEach((section) => {
    if (section.facet === "sites_buildings") {
      const max_sites_buildings = sites_buildings_expanded.value ? Infinity : section.n_visible;
      visible.push(...section.entries.slice(0, max_sites_buildings).map((e) => e.id));
    } else visible.push(...section.entries.map((e) => e.id));
  });
  return visible;
});

function searchFocus(): void {
  searchBarFocused.value = true;
  highlighted.value = undefined;
}

function searchBlur(): void {
  if (keep_focus.value) {
    setTimeout(() => {
      // This is relevant if the call is delayed and focused has
      // already been disabled e.g. when clicking on an entry.
      if (searchBarFocused.value) document.getElementById("search")?.focus();
    }, 0);
    keep_focus.value = false;
  } else {
    searchBarFocused.value = false;
  }
}

async function searchGo(cleanQuery: boolean): Promise<void> {
  if (query.value.length === 0) return;

  await navigateTo(localePath(`/search?q=${query.value}`));
  searchBarFocused.value = false;
  if (cleanQuery) {
    query.value = "";
  }
  document.getElementById("search")?.blur();
}

async function searchGoTo(id: string): Promise<void> {
  await navigateTo(localePath(`/view/${id}`));
  searchBarFocused.value = false;
  query.value = "";
  document.getElementById("search")?.blur();
}

function onKeyDown(e: KeyboardEvent): void {
  switch (e.key) {
    case "Escape":
      document.getElementById("search")?.blur();
      break;

    case "ArrowDown":
      if (highlighted.value === undefined) {
        highlighted.value = 0;
        e.preventDefault();
        break;
      }

      highlighted.value = (highlighted.value + 1) % visibleElements.value.length;
      e.preventDefault();
      break;

    case "ArrowUp":
      if (visibleElements.value.length === 0) {
        highlighted.value = undefined;
        e.preventDefault();
        break;
      }
      if (highlighted.value === 0 || highlighted.value === undefined) {
        highlighted.value = visibleElements.value.length - 1;
      } else {
        highlighted.value -= 1;
      }
      e.preventDefault();
      break;

    case "Enter":
      if (highlighted.value !== undefined) searchGoTo(visibleElements.value[highlighted.value]!);
      else searchGo(false);
      break;
    default:
      break;
  }
}

const runtimeConfig = useRuntimeConfig();
const url = computed(() => {
  const params = new URLSearchParams();
  params.append("q", query.value);
  params.append("lang", locale.value);
  params.append("pre_highlight", "<b class='text-blue'>");
  params.append("post_highlight", "</b>");

  return `${runtimeConfig.public.apiURL}/api/search?${params.toString()}`;
});
const { data, error } = await useFetch<SearchResponse>(url, {
  key: "search",
  dedupe: "cancel",
  credentials: "omit",
  retry: 120,
  retryDelay: 1000,
});
</script>

<template>
  <form action="/search" autocomplete="off" method="GET" role="search" class="flex flex-row" @submit="searchGo(false)">
    <div
      class="flex grow flex-row rounded-s-sm border border-zinc-400 bg-zinc-200 focus-within:outline focus-within:outline-2 focus-within:outline-offset-1 focus-within:outline-blue-600"
    >
      <textarea
        id="search"
        v-model="query"
        cols="1"
        rows="1"
        :title="t('input.aria-searchlabel')"
        aria-autocomplete="both"
        aria-haspopup="false"
        autocapitalize="off"
        autocomplete="off"
        spellcheck="false"
        maxlength="2048"
        name="q"
        type="text"
        class="grow resize-none bg-transparent py-2.5 ps-3 pe-5 font-semibold text-zinc-800 placeholder:font-normal placeholder:text-zinc-800 focus-within:placeholder:text-zinc-500 focus:outline-0"
        :placeholder="t('input.placeholder')"
        :aria-label="t('input.aria-searchlabel')"
        @focus="searchFocus"
        @blur="searchBlur"
        @keydown="onKeyDown"
      />
    </div>
    <button
      type="submit"
      class="rounded-e-sm bg-blue-500 px-3 py-1 text-xs font-semibold shadow-xs hover:bg-blue-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-blue-600"
      :aria-label="t('input.aria-actionlabel')"
      :title="t('input.action')"
    >
      <MagnifyingGlassIcon class="my-auto h-6 w-6 text-zinc-100" />
    </button>
  </form>
  <!-- Autocomplete -->
  <ClientOnly>
    <div
      v-if="searchBarFocused && data && query.length !== 0"
      class="shadow-4xl absolute top-3 mt-16 flex max-h-[calc(100vh-80px)] max-w-xl flex-col gap-4 overflow-auto rounded-sm border border-zinc-200 bg-zinc-50 p-3.5 shadow-zinc-700/30 md:-ms-2 md:me-3"
    >
      <!--
    <li class="search-comment filter">
      Suche einschränken auf:
      <NuxtLinkLocale class="bt btn-link btn-sm">Räume</NuxtLinkLocale>
    </li> -->
      <Toast v-if="error" id="search-error" level="error">
        <p class="text-md font-bold">{{ t("error.header") }}</p>
        <p class="text-sm">
          {{ t("error.reason") }}:<br />
          <code
            class="mt-2 mb-1 inline-flex max-w-full items-center space-x-2 overflow-auto rounded-md bg-red-200 px-4 py-3 text-left font-mono text-xs text-red-900 dark:bg-red-50/20"
          >
            {{ error }}
          </code>
        </p>
        <p class="text-sm">{{ t("error.call_to_action") }}</p>
      </Toast>
      <ul v-for="s in data.sections" v-cloak :key="s.facet" class="flex flex-col gap-2">
        <div class="flex items-center">
          <span class="text-md me-4 shrink text-zinc-800">{{ t(`sections.${s.facet}`) }}</span>
          <div class="grow border-t border-zinc-800" />
        </div>

        <template v-for="(e, i) in s.entries" :key="e.id">
          <SearchResultItemLink
            v-if="i < s.n_visible"
            :highlighted="e.id === visibleElements[highlighted ?? -1]"
            :item="e"
            @click="searchBarFocused = false"
            @mousedown="keep_focus = true"
            @mouseover="highlighted = undefined"
          />
        </template>
        <li class="-mt-2">
          <Btn
            v-if="s.facet === 'sites_buildings' && !sites_buildings_expanded && s.n_visible < s.entries.length"
            variant="linkButton"
            size="sm"
            @mousedown="keep_focus = true"
            @click="sites_buildings_expanded = true"
          >
            {{ t("show_hidden", s.entries.length - s.n_visible) }}
          </Btn>
          <span class="text-sm text-zinc-400">
            {{
              s.estimatedTotalHits > 20 ? t("approx_results", s.estimatedTotalHits) : t("results", s.estimatedTotalHits)
            }}
          </span>
        </li>

        <!--
      <li class="search-comment actions">
        <Btn size="sm"><ChevronRightIcon class="h-4 w-4" /> in Gebäude Suchen</Btn>
        <Btn size="sm"><MapPinIcon class="h-4 w-4" /> Hörsäle</Btn>
        <Btn size="sm"><MapPinIcon class="h-4 w-4" /> Seminarräume</Btn>
      </li>

      <li class="divider" data-content="Veranstaltungen" />
      <li class="menu-item">
        <NuxtLinkLocale :to="/event/">
          <div class="tile">
            <div class="tile-icon">
              <ClockIcon class="h-4 w-4" />
            </div>
            <div class="tile-content">
              <span class="tile-title">
                Advanced Practical Course Games Engineering: Building Information Modeling (IN7106)
              </span>
              <small class="tile-subtitle text-zinc"> Übung mit 4 Gruppen </small>
            </div>
          </div>
        </NuxtLinkLocale>
        <div class="menu-badge" style="display: none">
          <label class="label label-primary">frei</label>
        </div>
      </li> -->
      </ul>
    </div>
  </ClientOnly>
</template>

<i18n lang="yaml">
de:
  input:
    placeholder: Suche
    aria-actionlabel: Suche nach dem im Suchfeld eingetragenen Raum
    aria-searchlabel: Suchfeld
    action: Go
  show_hidden: +{count} ausgeblendet
  sections:
    sites_buildings: Gebäude / Standorte
    rooms: Räume
  results: 1 Ergebnis | {count} Ergebnisse
  approx_results: ca. {count} Ergebnisse
en:
  input:
    placeholder: Search
    aria-actionlabel: Search for the room-query entered in the search field
    aria-searchlabel: Search-field
    action: Go
  show_hidden: +{count} hidden
  sections:
    sites_buildings: Buildings / Sites
    rooms: Rooms
  results: 1 result | {count} results
  approx_results: approx. {count} results
</i18n>
