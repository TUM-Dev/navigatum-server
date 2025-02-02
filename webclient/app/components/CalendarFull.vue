<script setup lang="ts">
import FullCalendar from "@fullcalendar/vue3";
import type { CalendarOptions, EventInput, EventSourceFuncArg } from "@fullcalendar/core";
import listPlugin from "@fullcalendar/list";
import timeGridPlugin from "@fullcalendar/timegrid";
import dayGridPlugin from "@fullcalendar/daygrid";
import type { components, operations } from "~/api_types";
import deLocale from "@fullcalendar/core/locales/de";
import enLocale from "@fullcalendar/core/locales/en-gb";

type CalendarResponse = operations["calendar_handler"]["responses"][200]["content"]["application/json"];
type CalendarBody = operations["calendar_handler"]["requestBody"]["content"]["application/json"];
type CalendarLocationResponse = components["schemas"]["CalendarLocationResponse"];

const props = defineProps<{ showing: readonly string[] }>();
defineExpose({ refetchEvents });
const runtimeConfig = useRuntimeConfig();
const { locale } = useI18n({ useScope: "local" });

const earliest_last_sync = defineModel<string | null>("earliest_last_sync");
const locations = defineModel<Map<string, CalendarLocationResponse>>("locations");

interface Color {
  backgroundColor: string;
  borderColor: string;
  textColor: string;
}

function colorForType(entry_type: "lecture" | "exercise" | "exam" | "barred" | "other"): Color | Record<string, never> {
  switch (entry_type) {
    case "lecture":
      return { backgroundColor: "#93bae6", borderColor: "#3070b3", textColor: "#13243e" };
    case "exam":
      return { backgroundColor: "#e6bbe2", borderColor: "#c56fb9", textColor: "#3f1837" };
    case "exercise":
      return { backgroundColor: "#fdba74", borderColor: "#f97316", textColor: "#431407" };
    case "other":
      return { backgroundColor: "#d4d4d8", borderColor: "#71717a", textColor: "#09090b" };
    case "barred":
      return { backgroundColor: "#fca5a5", borderColor: "#ef4444", textColor: "#450a0a" };
    default:
      return {};
  }
}

async function fetchEvents(arg: EventSourceFuncArg): Promise<EventInput[]> {
  const body: CalendarBody = {
    start_after: arg.startStr,
    end_before: arg.endStr,
    ids: props.showing,
  };
  const url = `${runtimeConfig.public.apiURL}/api/calendar`;
  const data = await $fetch<CalendarResponse>(url, {
    method: "POST",
    body: body,
    credentials: "omit",
    retry: 120,
    retryDelay: 1000,
    headers: { "Content-Type": "application/json" },
  });
  extractInfos(data);

  const items = [];
  const show_room_names = Object.keys(data).length > 1;
  for (const [k, v] of Object.entries(data)) {
    items.push(
      ...v.events.map((e) => {
        const title = locale.value == "de" ? e.title_de : e.title_en;
        const color = colorForType(e.entry_type);
        return {
          id: e.id.toString(),
          title: show_room_names ? `${k} ${title}` : title,
          start: new Date(e.start_at),
          end: new Date(e.end_at),
          ...color,
        };
      }),
    );
  }
  return items;
}

function extractInfos(data: CalendarResponse): void {
  const last_syncs: Date[] = [];
  for (const d of Object.values(data)) {
    if (d.location.last_calendar_scrape_at) {
      last_syncs.push(new Date(d.location.last_calendar_scrape_at));
    }
  }
  earliest_last_sync.value = last_syncs
    .reduce((d1, d2) => (d1 < d2 ? d1 : d2))
    .toLocaleString(locale.value, {
      timeStyle: "short",
      dateStyle: "short",
    });
  const tempLocationMap = new Map<string, CalendarLocationResponse>();
  for (const [key, v] of Object.entries(data)) {
    tempLocationMap.set(key, v.location);
  }
  locations.value = tempLocationMap;
}

const calendarOptions: CalendarOptions = {
  plugins: [timeGridPlugin, dayGridPlugin, listPlugin],
  initialView: "timeGridWeek",
  weekends: true,
  events: fetchEvents,
  headerToolbar: {
    left: "prev,next",
    center: "title",
    right: "dayGridMonth,timeGridWeek,timeGridDay,listWeek",
  },
  locale: locale.value == "de" ? deLocale : enLocale,
  height: 700,
  // like '14:30'
  views: {
    dayGridMonth: { displayEventEnd: false },
    timeGridDay: { displayEventEnd: false },
    timeGridWeek: { displayEventEnd: false },
    list: { displayEventEnd: true },
  },
  eventTimeFormat: {
    hour: "2-digit",
    minute: "2-digit",
    meridiem: false,
  },
};

const fullCalendarRef = ref<InstanceType<typeof FullCalendar> | null>(null);

function refetchEvents() {
  const api = fullCalendarRef.value?.getApi();
  if (api) {
    console.debug("Re-Fetching events");
    api.refetchEvents();
  } else {
    nextTick(refetchEvents);
  }
}
</script>

<template>
  <div class="flex max-h-[700px] min-h-[700px] grow flex-col">
    <FullCalendar :options="calendarOptions">
      <template #eventContent="arg">
        <NuxtLink
          :to="`https://campus.tum.de/tumonline/ee/ui/ca2/app/desktop/#/pl/ui/$ctx/!wbTermin.wbEdit?pTerminNr=${arg.event.id}`"
          external
          class="flex gap-1 overflow-x-auto overflow-y-auto"
          :class="['listWeek', 'timeGridWeek', 'timeGridDay'].includes(arg.view.type) ? 'flex-col' : 'flex-row'"
        >
          <template v-if="arg.view.type == 'timeGridWeek' || arg.view.type == 'timeGridDay'">
            <span class="font-medium">{{ arg.event.title }}</span>
            <span class="font-normal opacity-70"
              >{{ arg.timeText }} - {{ arg.event.end.toLocaleTimeString("de", { timeStyle: "short" }) }}</span
            >
          </template>
          <template v-else-if="arg.view.type == 'listWeek'">
            <span class="font-medium">{{ arg.event.title }}</span>
          </template>
          <template v-else>
            <span class="font-normal opacity-80">{{ arg.timeText }}</span>
            <span class="font-medium">{{ arg.event.title }}</span>
          </template>
        </NuxtLink>
      </template>
    </FullCalendar>
  </div>
</template>

<style lang="postcss">
@reference "../assets/main.css";

.fc-event-main {
  @apply flex;
}

.fc-list-day-side-text,
.fc-list-day-text {
  @apply text-zinc-900 dark:text-zinc-100;
}
</style>
