<script setup lang="ts">
import { FlagIcon } from "@heroicons/vue/16/solid";
import { useFeedback } from "~/composables/feedback";

const route = useRoute();
const feedback = useFeedback();
const { t } = useI18n({ useScope: "local" });
</script>

<template>
  <button
    type="button"
    class="focusable rounded-xs"
    :title="t('feedback')"
    data-cy="open-feedback-details"
    @click="
      () => {
        feedback.open = true;
        feedback.data = {
          category: 'entry',
          subject: `[${route.params.id}]: `,
          body: '',
          deletion_requested: false,
        };
      }
    "
  >
    <FlagIcon class="h-4 w-4 text-blue-600 hover:text-blue-900" />
  </button>
</template>

<i18n lang="yaml">
de:
  feedback: Problem melden oder Änderung vorschlagen
en:
  feedback: Report issue or suggest changes
</i18n>
