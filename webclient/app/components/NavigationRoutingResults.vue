<script setup lang="ts">
import type { operations } from "~/api_types";

type NavigationResponse = operations["route_handler"]["responses"][200]["content"]["application/json"];
defineProps<{ data: NavigationResponse }>();
const emit = defineEmits<{
  selectManeuver: [id: { begin_shape_index: number; end_shape_index: number }];
}>();
const { t } = useI18n({ useScope: "local" });
</script>

<template>
  <div>
    <div v-for="(l, i) in data.legs" :key="i" class="gap-1">
      <p class="mt-3 flex items-center gap-5 pb-4 font-semibold text-zinc-500">
        <span>{{
          l.summary.length_meters >= 1000
            ? t("kilometers", [(l.summary.length_meters / 1000).toFixed(1)])
            : t("meters", l.summary.length_meters)
        }}</span>
        <span class="grow border-t border-zinc-500" />
        <span>{{
          l.summary.time_seconds >= 60
            ? t("minutes", Math.ceil(l.summary.time_seconds / 60))
            : t("seconds", l.summary.time_seconds)
        }}</span>
      </p>
      <div
        v-for="(m, j) in l.maneuvers"
        :key="j"
        class="group cursor-pointer py-1"
        @click="emit('selectManeuver', { begin_shape_index: m.begin_shape_index, end_shape_index: m.end_shape_index })"
      >
        <div
<<<<<<< HEAD
          class="bg-zinc-200 flex flex-row items-center gap-3 overflow-auto rounded-md p-2 py-1 group-hover:bg-zinc-300"
=======
          class="flex flex-row items-center gap-3 rounded-md bg-zinc-200 p-2 py-1 group-hover:bg-zinc-300"
>>>>>>> 73d37b89 (intial migration to tailwind)
          :aria-label="m.verbal_transition_alert_instruction ?? undefined"
        >
          <NavigationRoutingManeuverIcon :type="m.type" />
          <div class="text-zinc-900">{{ m.instruction }}</div>
        </div>
        <small
          v-if="m.length_meters"
          class="text-zinc-500"
          :aria-label="m.verbal_post_transition_instruction ?? undefined"
          >{{ t("meters", m.length_meters) }}</small
        >
      </div>
    </div>
  </div>
</template>

<i18n lang="yaml">
de:
  minutes: "sofort | eine Minute | {count} Minuten"
  seconds: "sofort | eine Sekunde | {count} Sekunden"
  meters: "hier | einen Meter | {count} Meter"
  kilometers: "{0} Kilometer"
en:
  minutes: "instant | one minute | {count} minutes"
  seconds: "instant | one second | {count} seconds"
  meters: "here | one meter | {count} meters"
  kilometers: "{0} kilometers"
</i18n>
