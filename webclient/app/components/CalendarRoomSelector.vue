<script setup lang="ts">
import { PlusCircleIcon } from "@heroicons/vue/24/outline";
import { CalendarIcon } from "@heroicons/vue/16/solid";
import type { components } from "~/api_types";
import PreviewIcon from "~/components/PreviewIcon.vue";
import { useCalendar } from "~/composables/calendar";

type LocationDetailsResponse = components["schemas"]["LocationDetailsResponse"];
type CalendarLocationResponse = components["schemas"]["CalendarLocationResponse"];

defineProps<{
  readonly data: Map<string, CalendarLocationResponse>;
}>();
const emit = defineEmits(["change"]);

const { t } = useI18n({ useScope: "local" });
const runtimeConfig = useRuntimeConfig();
const calendar = useCalendar();

async function addLocation() {
  let selectedLocation: string = "";
  while (!selectedLocation) {
    selectedLocation = window.prompt(t("prompt.initial")) || "";
    try {
      const result = await fetch(`${runtimeConfig.public.apiURL}/api/locations/${selectedLocation}`);
      if (!result.ok) {
        const userWantsToRetry = window.confirm(t("prompt.error_not_ok", [selectedLocation]));
        if (!userWantsToRetry) return;
        selectedLocation = "";
        continue;
      }
      const res = (await result.json()) as LocationDetailsResponse;
      if (!res.props.calendar_url) {
        const userWantsToRetry = window.confirm(t("prompt.error_not_calendar", [selectedLocation]));
        if (!userWantsToRetry) return;
        selectedLocation = "";
        continue;
      }
    } catch (e) {
      window.alert("Failed because " + e);
      selectedLocation = "";
      continue;
    }
    if (calendar.value.find((k) => k == selectedLocation)) {
      const userWantsToRetry = window.confirm(t("prompt.error_already_exists", [selectedLocation]));
      if (!userWantsToRetry) return;
      selectedLocation = "";
    }
  }
  calendar.value = [...calendar.value, selectedLocation];
  emit("change");
  // todo: debug why this is not syncinc apropriately, quite a crude hack
  setTimeout(() => location.reload(), 500);
}
</script>

<template>
  <ul v-if="calendar.length" class="mb-6 flex gap-2 overflow-x-auto">
    <li
      v-for="[key, location] in data.entries()"
      :key="key"
      class="flex min-w-64 gap-1 rounded-md px-2 md:gap-3 md:px-4"
    >
      <PreviewIcon
        :item="{
          type: location.type,
          parsed_id: 'parsed_id' in location ? (location.parsed_id as string) : undefined,
        }"
      />
      <div class="flex flex-col gap-1 text-zinc-600">
        <NuxtLinkLocale class="line-clamp-1 hover:underline" :to="'/view/' + key">{{ location.name }}</NuxtLinkLocale>
        <small>
          {{ location.type_common_name }}
        </small>
        <small>
          <Btn :to="location.calendar_url" variant="link" size="text-xs font-semibold rounded-md">
            <CalendarIcon class="mb-0.5 h-4 w-4" />
            {{ t("view_in_tumonline") }}
          </Btn>
        </small>
      </div>
    </li>
    <li class="flex">
      <button
        type="button"
        class="focusable group flex min-w-14 origin-center place-items-center content-center items-center justify-center justify-items-center justify-self-center rounded-md border border-zinc-200 bg-zinc-50 object-center align-middle hover:bg-blue-100"
        :title="t('add_location')"
        :aria-label="t('add_location')"
        @click="addLocation"
      >
        <PlusCircleIcon class="h-5 w-5 group-hover:text-black" />
      </button>
    </li>
  </ul>
</template>

<i18n lang="yaml">
de:
  add_location: Zusätzliche Location zum Kalendar hinzufügen
  view_in_tumonline: in TUMonline ansehen
  prompt:
    initial: |-
      Welcher Kalender soll hinzugefügt werden?
      Bitte wähle nur den letzten Teil der Url eines Raumes mit einem Kalender.

      Beispiel:
      https://nav.tum.de/room/5502.01.250 hat die ID 5502.01.250
    error_not_ok: |-
      Der Ort '{0}' existiert nicht.
      Wenn er existieren würde, gäbe es https://nav.tum.de/view/{0}.

      Möchtest du es mit einer anderen ID erneut versuchen?
    error_not_calendar: |-
      Der Ort hat keinen Kalender.
      Möchtest du es mit einer anderen ID erneut versuchen?
    error_already_exists: |-
      Der Ort ist bereits im Kalender eingetragen.
      Möchtest du es mit einer anderen ID erneut versuchen?
en:
  add_location: add additional location to the calendar
  view_in_tumonline: View in TUMonline
  prompt:
    initial: |-
      Which calendar should be added?
      Please select just the last part of the url of a room with a calendar.

      Example:
      https://nav.tum.de/room/5502.01.250 has the id 5502.01.250
    error_not_ok: |-
      Location '{0}' does not exist.
      If it were to exist https://nav.tum.de/view/{0} would exist.
      Want to retry with a different id?
    error_not_calendar: |-
      Location does not have a calendar.
      Want to retry with a different id?
    error_already_exists: |-
      Location is already in the calendar.
      Want to retry with a different id?
</i18n>
