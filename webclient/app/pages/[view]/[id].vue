<script setup lang="ts">
import { useClipboard } from "@vueuse/core";
import type { components } from "~/api_types";
import { Tab, TabGroup, TabList, TabPanel, TabPanels } from "@headlessui/vue";
import { CalendarDaysIcon } from "@heroicons/vue/16/solid";
import { ClipboardDocumentCheckIcon, LinkIcon } from "@heroicons/vue/20/solid";
import type { DetailsFeedbackButton, DetailsInteractiveMap } from "#components";
import { useRouteQuery } from "@vueuse/router";

definePageMeta({
  validate(route) {
    return /(view|campus|site|building|room|poi)/.test(route.params.view as string);
  },
  layout: "details",
});

type LocationDetailsResponse = components["schemas"]["LocationDetailsResponse"];
type ImageInfoResponse = components["schemas"]["ImageInfoResponse"];

const { t, locale } = useI18n({ useScope: "local" });
const localePath = useLocalePath();
const route = useRoute();
const router = useRouter();

const calendar = useCalendar();
const runtimeConfig = useRuntimeConfig();
const url = computed(() => `${runtimeConfig.public.apiURL}/api/locations/${route.params.id}?lang=${locale.value}`);
const { data, error } = useFetch<LocationDetailsResponse, string>(url, {
  key: "details",
  dedupe: "cancel",
  credentials: "omit",
  retry: 120,
  retryDelay: 1000,
});

const shownImage = ref<ImageInfoResponse | undefined>(data.value?.imgs?.length ? data.value.imgs[0] : undefined);
const slideshowOpen = ref(false);

const clipboardSource = computed(() => `https://nav.tum.de${route.fullPath}`);
const { copy, copied, isSupported: clipboardIsSupported } = useClipboard({ source: clipboardSource });

const selectedMap = useRouteQuery<"interactive" | "plans">("map", "interactive", {
  mode: "replace",
  route,
  router,
  transform: (val) => (val === "plans" ? "plans" : "interactive"),
});

watchEffect(async () => {
  if (route.params.id === "root") {
    await navigateTo({ path: localePath("/"), replace: true });
  }
});
watchEffect(async () => {
  if (error.value) {
    await navigateTo({
      path: "/404",
      query: { ...route.query, path: route.path },
      hash: route.hash,
      replace: true,
    });
  }
});
watch([data, route], async () => {
  if (!data.value) return;
  const redirectPath = localePath(data.value.redirect_url);
  if (route.path !== redirectPath) {
    await navigateTo({ path: redirectPath, query: route.query, replace: true });
  }
});
watch([data], () => {
  if (!data.value) return;
  // --- Additional data ---
  slideshowOpen.value = false;
  // --- Images ---
  shownImage.value = data.value.imgs?.length ? data.value.imgs[0] : undefined;
});

const description = computed(() => {
  if (data.value === undefined || data.value === null) return "";

  let description = t("details_for");
  if (data.value.name.includes(data.value.type_common_name)) {
    description += ` ${data.value.name}`;
  } else {
    description += ` ${data.value.type_common_name} ${data.value.name}`;
  }
  if (data.value.props.computed) {
    description += ":";
    data.value.props.computed.forEach((prop) => {
      description += `\n- ${prop.name}: ${prop.text}`;
    });
  }
  return description;
});
const title = computed(() => data.value?.name || route.params.id + " - Navigatum");
useSeoMeta({
  title: title,
  ogTitle: title,
  description: description,
  ogDescription: description,
  ogImage: `https://nav.tum.de/api/locations/${route.params.id}/preview`,
  twitterCard: "summary_large_image",
});
</script>

<template>
  <div v-if="data" class="flex flex-col gap-5">
    <!-- Header image (on mobile) -->
    <button
      v-if="data.imgs?.length && data.imgs[0]"
      type="button"
      class="focusable block lg:hidden print:!hidden"
      @click="slideshowOpen = true"
    >
      <NuxtImg
        width="256"
        height="105"
        :alt="t('image_alt')"
        :src="`${runtimeConfig.public.cdnURL}/cdn/lg/${data.imgs[0].name}`"
        sizes="1024px sm:256px md:512px"
        densities="x1 x2"
        class="block w-full"
        preload
        :placeholder="[256, 105]"
      />
    </button>

    <!-- Entry header / title -->
    <div class="px-5">
      <BreadcrumbList
        :items="
          data.parent_names.map((n, i) => ({
            name: n,
            to: i > 0 ? '/view/' + data?.parents[i] : '/',
          }))
        "
        class="pt-6 pb-3"
      />
      <div class="group flex flex-row gap-2">
        <button
          v-if="clipboardIsSupported"
          :title="t('header.copy_link')"
          type="button"
          tabindex="1"
          class="-ms-8 hidden px-1 text-transparent transition-colors group-hover:text-zinc-800 focus:text-zinc-800 lg:block"
          @click="copy(`https://nav.tum.de${route.fullPath}`)"
        >
          <ClipboardDocumentCheckIcon v-if="copied" class="h-4 w-4" />
          <LinkIcon v-else class="h-4 w-4" />
        </button>
        <h1 class="text-xl font-bold text-zinc-700">{{ data.name }}</h1>
      </div>
      <div>
        <div class="flex grow place-items-center justify-between">
          <span class="mt-0.5 text-sm text-zinc-500">{{ data.type_common_name }}</span>
          <div class="flex flex-row place-items-center gap-3">
            <button
              v-if="data.props?.calendar_url"
              type="button"
              class="focusable rounded-xs"
              :title="t('header.calendar')"
              @click="calendar = [...new Set([...calendar, route.params.id?.toString() ?? '404'])]"
            >
              <CalendarDaysIcon class="mt-0.5 h-4 w-4 text-blue-600" />
            </button>
            <ShareButton :coords="data.coords" :name="data.name" />
            <DetailsFeedbackButton />
            <!-- <button class="btn btn-link btn-action btn-sm"
                  :title="t('header.favorites')">
            <BookmarkIcon class="w-4 h-4" v-if="bookmarked" />
            <BookmarkSquareIcon class="w-4 h-4" v-else />
          </button> -->
          </div>
        </div>
      </div>
    </div>

    <!-- First info section (map + infocard) -->
    <div class="grid grid-cols-1 gap-5 px-5 lg:grid-cols-3">
      <TabGroup
        class="col-span-1 lg:col-span-2"
        as="div"
        :selected-index="selectedMap === 'interactive' ? 0 : 1"
        :default-index="selectedMap === 'interactive' ? 0 : 1"
        @change="(index) => (selectedMap = index === 0 ? 'interactive' : 'plans')"
      >
        <div class="mb-3 grid gap-2 lg:hidden">
          <Toast
            v-if="data.type === 'room' && data.maps?.overlays?.default === null"
            level="warning"
            :msg="t('no_floor_overlay')"
          />
          <Toast v-if="data.props.comment" :msg="data.props.comment" />
        </div>
        <TabPanels>
          <TabPanel id="interactiveMapPanel" :tab-index="0" :unmount="false">
            <ClientOnly>
              <DetailsInteractiveMap
                :id="data.id"
                :coords="data.coords"
                :type="data.type"
                :maps="data.maps"
                :debug-mode="!!route.query.debug"
              />
            </ClientOnly>
          </TabPanel>
          <TabPanel id="plansMapPanel" :tab-index="1">
            <ClientOnly>
              <LazyDetailsRoomfinderMap
                v-if="data.maps.roomfinder?.available"
                :available="data.maps.roomfinder.available"
                :default-map-id="data.maps.roomfinder.default"
              />
            </ClientOnly>
          </TabPanel>
        </TabPanels>
        <TabList class="flex space-x-1 rounded-md bg-zinc-100 p-1 print:!hidden">
          <Tab :tab-index="0" as="template" @click="selectedMap = 'interactive'">
            <button
              type="button"
              class="focusable w-full rounded-md py-2.5 text-sm leading-5 font-medium"
              :class="[
                selectedMap === 'interactive'
                  ? 'bg-zinc-300 text-zinc-900 shadow'
                  : 'bg-zinc-300/5 text-zinc-800 hover:bg-zinc-500/20 hover:text-zinc-900',
              ]"
            >
              {{ t("map.interactive") }}
            </button>
          </Tab>
          <Tab :tab-index="1" as="template" :disabled="!data.maps.roomfinder?.available" @click="selectedMap = 'plans'">
            <button
              type="button"
              class="focusable w-full rounded-md py-2.5 text-sm leading-5 font-medium"
              :class="{
                'bg-zinc-300 text-zinc-900 shadow': selectedMap === 'plans',
                'bg-zinc-300/5 text-zinc-800': selectedMap !== 'plans',
                'hover:bg-zinc-500/20 hover:text-zinc-900': data.maps.roomfinder?.available,
                'cursor-not-allowed !text-zinc-400': !data.maps.roomfinder?.available,
              }"
            >
              {{ t("map.plans") }}
            </button>
          </Tab>
        </TabList>
      </TabGroup>
      <!-- Map container -->

      <DetailsInfoSection v-model:shown_image="shownImage" v-model:slideshow_open="slideshowOpen" :data="data" />
    </div>

    <DetailsBuildingOverviewSection :buildings="data.sections?.buildings_overview" />
    <ClientOnly>
      <LazyDetailsRoomOverviewSection :rooms="data.sections?.rooms_overview" />
    </ClientOnly>
    <DetailsSources
      :coords="data.coords"
      :sources="data.sources"
      :image="data.imgs?.length ? data.imgs[0] : undefined"
    />
  </div>
  <div v-else class="flex flex-col items-center gap-5 py-32 text-zinc-900">
    <Spinner class="h-8 w-8" />
    {{ t("Loading data...") }}
  </div>
  <ClientOnly>
    <LazyCalendarModal v-if="calendar.length" />
  </ClientOnly>
</template>

<i18n lang="yaml">
de:
  image_alt: Header-Bild, zeigt das Gebäude
  details_for: Details für
  map:
    interactive: Interaktive Karte
    plans: Lagepläne
  no_floor_overlay: Für den angezeigten Raum gibt es leider keine Indoor Karte.
  header:
    calendar: Kalender öffnen
    copy_link: Link kopieren
    favorites: Zu Favoriten hinzufügen
  Loading data...: Lädt daten...
en:
  image_alt: Header image, showing the building
  details_for: Details for
  map:
    interactive: Interactive Map
    plans: Site Plans
  no_floor_overlay: There is unfortunately no indoor map for the displayed room.
  header:
    calendar: Open calendar
    copy_link: Copy link
    favorites: Add to favorites
  Loading data...: Loading data...
</i18n>
