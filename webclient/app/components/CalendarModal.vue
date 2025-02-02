<script setup lang="ts">
import { useFeedback } from "~/composables/feedback";
import { useCalendar } from "~/composables/calendar";
import type { components } from "~/api_types";
import type { CalendarFull } from "#components";

type CalendarLocationResponse = components["schemas"]["CalendarLocationResponse"];

const feedback = useFeedback();
const calendar = useCalendar();
const { t } = useI18n({ useScope: "local" });
// all the below are updated by the calendar
const earliest_last_sync = ref<string | null>(null);
const locations = ref<Map<string, CalendarLocationResponse>>(new Map());
const modalOpen = ref(!!calendar.value.length);
watchEffect(() => {
  if (!!calendar.value.length && !modalOpen.value) {
    modalOpen.value = true;
  }
});
const fullCalendarRef = ref<InstanceType<typeof CalendarFull> | null>(null);
</script>

<template>
  <Modal v-model="modalOpen" :title="t('title')" class="!min-w-[90vw]" @close="calendar = []">
    <NuxtErrorBoundary>
      <template #error="{ error }">
        <Toast level="error">
          <p class="text-md font-bold">{{ t("error.header") }}</p>
          <p class="text-sm">
            {{ t("error.reason") }}:<br />
            <code
              class="mt-2 mb-1 inline-flex max-w-full items-center space-x-2 overflow-auto rounded-md bg-red-200 px-4 py-3 text-left font-mono text-xs text-red-900 dark:bg-red-50/20"
            >
              {{ error }}
            </code>
          </p>
          <I18nT class="text-sm" tag="p" keypath="error.call_to_action">
            <template #feedbackForm>
              <button
                type="button"
                class="bg-transparent text-blue-600 visited:text-blue-600 hover:underline"
                :aria-label="t('error.feedback-open')"
                @click="
                  () => {
                    feedback.open = true;
                    feedback.data = {
                      category: 'bug',
                      subject: 'calendar error',
                      body: `While viewing the calendar for ${JSON.stringify(calendar)}
I got this error:
\`\`\`
${error}
\`\`\`

In case you have trouble replicating this bug, my environment is PLEASE_INSERT_YOUR_BROWSER_HERE.
I also did PLEASE_INSERT_IF_YOU_DID_SOMETHING_SPECIAL_BEFOREHAND`,
                      deletion_requested: false,
                    };
                  }
                "
              >
                {{ t("error.feedback-form") }}
              </button>
            </template>
          </I18nT>
        </Toast>
      </template>
      <template #default>
        <div>
          <div v-if="locations.size === 0" class="flex flex-col items-center gap-5 py-32 text-zinc-900">
            <Spinner class="h-8 w-8" />
            {{ t("Loading data...") }}
          </div>
          <div :class="{ '!invisible': locations.size === 0 }">
            <CalendarRoomSelector :data="locations" @change="fullCalendarRef?.refetchEvents()" />
            <CalendarFull
              v-model:earliest_last_sync="earliest_last_sync"
              v-model:locations="locations"
              :showing="calendar"
            />
            <p class="pt-2 text-xs">
              {{ t("footer.disclaimer") }} <br />
              {{ t("footer.please_check") }}
              <template v-if="earliest_last_sync !== null">
                <br />
                {{ t("footer.last_sync", [earliest_last_sync]) }}
              </template>
            </p>
          </div>
        </div>
      </template>
    </NuxtErrorBoundary>
  </Modal>
</template>

<i18n lang="yaml">
de:
  title: Kalendar
  close: Schließen
  error:
    header: Beim Versuch, den Kalender anzuzeigen, ist ein Fehler aufgetreten
    reason: Der Grund für diesen Fehler ist
    call_to_action: Wenn dieses Problem weiterhin besteht, kontaktieren Sie uns bitte über das {feedbackForm}.
    feedback-form: Feedback-Formular
    feedback-open: Feedback-Formular öffnen
  footer:
    disclaimer: Stündlich aktualisiert und identische Termine zusammengefasst.
    please_check: Im Zweifelsfall prüfe bitte den offiziellen TUMonline-Kalender.
    last_sync: Stand {0}
  Loading data...: Lädt daten...
en:
  title: Calendar
  close: Close
  error:
    header: Got an error trying to display calendar
    reason: Reason for this error is
    call_to_action: If this issue persists, please contact us via the {feedbackForm}.
    feedback-form: feedback form
    feedback-open: open the feedback form
  footer:
    disclaimer: Updated hourly and identical events are merged.
    please_check: If in doubt, please check the official calendar on TUMonline
    last_sync: Updated {0}
  Loading data...: Loading data...
</i18n>
