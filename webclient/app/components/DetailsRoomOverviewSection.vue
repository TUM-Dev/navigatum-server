<script setup lang="ts">
import { Listbox, ListboxButton, ListboxOption, ListboxOptions } from "@headlessui/vue";
import { CheckIcon, ChevronUpDownIcon, FunnelIcon, MagnifyingGlassIcon, MapPinIcon } from "@heroicons/vue/24/outline";
import type { components } from "~/api_types";
import { useVirtualList } from "@vueuse/core";

type RoomsOverviewResponse = components["schemas"]["RoomsOverviewResponse"];
type RoomsOverviewUsageChildResponse = components["schemas"]["RoomsOverviewUsageChildResponse"];

const props = defineProps<{
  readonly rooms?: RoomsOverviewResponse | null;
}>();

const { t } = useI18n({ useScope: "local" });
const selectedUsage = ref(-1);
const search = ref("");
const combined_list = computed(() => {
  const usages = props.rooms?.usages || [];
  const combinedList = [] as RoomsOverviewUsageChildResponse[];
  usages.forEach((usage) => {
    combinedList.push(...usage.children);
  });
  return combinedList;
});
type SelectedRoomGroup = {
  rooms: readonly RoomsOverviewUsageChildResponse[];
  label: string;
};
const selectedRooms = computed<SelectedRoomGroup>(() => {
  if (selectedUsage.value === -1) {
    return { rooms: combined_list.value, label: t("any_usage") };
  }
  const rooms_usgage = props.rooms?.usages || [];
  return {
    rooms: rooms_usgage[selectedUsage.value]?.children ?? [],
    label: rooms_usgage[selectedUsage.value]?.name ?? "usage-out-of-range",
  };
});
const filteredList = computed<RoomsOverviewUsageChildResponse[]>(() => {
  const search_term = new RegExp(`.*${search.value}.*`, "i"); // i ^= case-insensitive
  return selectedRooms.value.rooms.filter((f) => search_term.test(f.name));
});
const { list, containerProps, wrapperProps } = useVirtualList<RoomsOverviewUsageChildResponse>(filteredList, {
  itemHeight: 36,
});
</script>

<template>
  <div
    v-if="props.rooms?.usages"
    class="flex flex-col gap-3 p-4 md:mx-5 md:rounded-sm md:border md:border-zinc-300 md:bg-white md:dark:bg-zinc-100 print:!hidden"
  >
    <p class="text-lg font-semibold text-zinc-800">{{ t("title") }}</p>
    <div class="flex flex-col gap-2">
      <Listbox v-model="selectedUsage" as="div" class="relative z-10">
        <ListboxButton
          class="focusable relative w-full rounded-xs border border-zinc-400 bg-zinc-200 py-2 pr-10 text-left text-zinc-600 sm:text-sm"
        >
          <span class="absolute inset-y-0 left-0 flex items-center pl-2">
            <FunnelIcon class="h-4 w-4" />
          </span>
          <span class="block truncate ps-8">{{ selectedRooms.label }}</span>
          <span class="absolute inset-y-0 right-0 flex items-center pr-2">
            <ChevronUpDownIcon class="h-5 w-5" />
          </span>
        </ListboxButton>

        <Transition
          leave-active-class="transition duration-100 ease-in"
          leave-from-class="opacity-100"
          leave-to-class="opacity-0"
        >
          <ListboxOptions
            class="absolute !m-0 mt-1 max-h-60 w-full overflow-auto rounded-md bg-zinc-200 text-base ring-1 shadow-lg ring-black/5 focus:outline-hidden sm:text-sm"
          >
            <ListboxOption
              v-slot="{ active, selected }"
              :key="-1"
              :value="-1"
              as="li"
              class="cursor-pointer list-none select-none"
            >
              <div
                class="flex flex-row justify-start gap-3 px-3 py-2"
                :class="[active ? 'bg-blue-100 text-blue-900' : 'text-zinc-900']"
              >
                <span v-if="selected" class="my-auto text-blue-600">
                  <CheckIcon class="h-5 w-5" />
                </span>
                <div class="flex grow flex-row justify-between gap-3">
                  <span
                    class="my-auto block truncate text-zinc-600"
                    :class="[selected ? 'font-medium' : 'ms-10 font-normal']"
                  >
                    {{ t("any_usage") }}
                  </span>
                  <span class="rounded-md bg-blue-300 px-2 py-1 text-sm text-blue-950"
                    >{{ t("rooms", combined_list.length) }}
                  </span>
                </div>
              </div>
            </ListboxOption>
            <ListboxOption
              v-for="(usage, i) in props.rooms.usages"
              v-slot="{ active, selected }"
              :key="i"
              :value="i"
              as="li"
              class="cursor-pointer list-none select-none"
            >
              <div
                class="flex flex-row justify-start gap-3 px-3 py-2"
                :class="[active ? 'bg-blue-100 text-blue-900' : 'text-zinc-900']"
              >
                <span v-if="selected" class="my-auto text-blue-600">
                  <CheckIcon class="h-5 w-5" />
                </span>
                <div class="flex grow flex-row justify-between gap-3">
                  <span
                    class="my-auto block truncate text-zinc-600"
                    :class="[selected ? 'font-medium' : 'ms-10 font-normal']"
                  >
                    {{ usage.name }}
                  </span>
                  <span class="rounded-md bg-blue-300 px-2 py-1 text-sm text-blue-950"
                    >{{ t("rooms", usage.count) }}
                  </span>
                </div>
              </div>
            </ListboxOption>
          </ListboxOptions>
        </Transition>
      </Listbox>
      <div class="z-0 flex w-full shrink items-center border border-zinc-400 bg-zinc-200">
        <MagnifyingGlassIcon class="h-4 w-6 pl-2 text-zinc-600" aria-hidden="true" />
        <textarea
          id="search-input"
          v-model="search"
          :title="t('search')"
          :aria-label="t('search_rooms_of_building')"
          rows="1"
          aria-autocomplete="both"
          aria-haspopup="false"
          autocomplete="off"
          autocapitalize="off"
          spellcheck="false"
          maxlength="2048"
          type="text"
          class="focusable w-full grow resize-none rounded-xs bg-zinc-200 py-2 ps-6 font-semibold text-zinc-800 placeholder:font-normal placeholder:text-zinc-800 focus-within:placeholder:text-zinc-500"
          :placeholder="t('search')"
        />
      </div>
    </div>
    <div class="text-zinc-600">
      <div
        v-if="filteredList.length > 0"
        v-bind="containerProps"
        class="max-h-96 overflow-y-scroll border border-zinc-400 bg-zinc-100 p-2 dark:bg-zinc-200"
      >
        <ul v-bind="wrapperProps">
          <li>
            <NuxtLinkLocale
              v-for="(room, index) in list"
              :key="index"
              :to="`/view/${room.data.id}`"
              class="flex h-[36px] max-h-[36px] min-h-[36px] flex-row gap-2 p-1.5 px-3 hover:bg-blue-500 hover:text-white"
              external
            >
              <MapPinIcon class="my-auto h-4 w-4" aria-hidden="true" />
              {{ room.data.name }}
            </NuxtLinkLocale>
          </li>
        </ul>
      </div>
      <div v-else class="flex flex-row items-baseline">
        {{ t("no_results_with_these_filters") }}
        <Btn
          size="sm"
          variant="linkButton"
          @click="
            () => {
              search = '';
              selectedUsage = -1;
            }
          "
          >{{ t("clear_filter") }}
        </Btn>
      </div>
      <small class="p-4">
        {{ t("results", filteredList.length) }}
      </small>
    </div>
  </div>
</template>

<i18n lang="yaml">
de:
  any_usage: beliebige Nutzung
  filter_by_usage: nach Nutzung filtern
  no_results_with_these_filters: Keine Ergebnisse mit diesen Filtern gefunden.
  clear_filter: Filter löschen
  search: durchsuchen
  results: 1 Ergebnis | {count} Ergebnisse
  rooms: 1 Raum | {count} Räume
  title: Räume
  search_rooms_of_building: durchsucht Räume des Gebäudes
en:
  any_usage: any usage
  filter_by_usage: filter by usage
  no_results_with_these_filters: No results found with these filters.
  clear_filter: Clear the filters
  search: search
  results: "{count} result | {count} results"
  rooms: 1 room | {count} rooms
  title: Rooms
  search_rooms_of_building: searches rooms of the building
</i18n>
