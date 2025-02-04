<script setup lang="ts">
import { ArrowRightIcon } from "@heroicons/vue/24/outline";

const route = useRoute();

const feedback = useFeedback();
const { t } = useI18n({ useScope: "local" });
</script>

<template>
  <div class="mx-auto max-w-xl pt-4">
    <img src="../assets/404_navigatum.svg" :alt="t('img_alt')" />
    <div class="flex flex-col items-center gap-1 p-5">
      <h5 class="text-lg text-zinc-800">{{ t("header") }}</h5>
      <p class="text-md text-zinc-600">{{ t("description") }}</p>
    </div>
    <div class="flex flex-row items-center justify-evenly">
      <div class="flex flex-row gap-4">
        <Btn to="/" variant="primary">
          {{ t("go_home") }}
        </Btn>
        <Btn
          variant="linkButton"
          @click="
            () => {
              feedback.open = true;
              feedback.data = {
                category: 'bug',
                subject: `404 on \`${route.fullPath}\``,
                body: t('got_here'),
                deletion_requested: false,
              };
            }
          "
        >
          {{ t("call_to_action") }}
          <ArrowRightIcon class="mt-0.5 h-4 w-4" />
        </Btn>
      </div>
    </div>
  </div>
</template>

<i18n lang="yaml">
de:
  open_feedback: Open the feedback-form
  description: Dies könnte sein, weil wir einen Fehler gemacht haben.
  got_here: "Ich habe diesen Fehler so gefunden:\r\n1. ..."
  header: Die angeforderte Seite wurde nicht gefunden.
  img_alt: Illustration einer männlichen Person, die auf großen '404'-Buchstaben sitzt und einer weiblichen person vor dem TUM Hauptgebäude
  go_home: Zur Startseite
  call_to_action: Feedback geben
en:
  open_feedback: Open the feedback-form
  description: This could be because we made a mistake.
  got_here: "I have found the error by:\r\n1. ..."
  header: The requested website could not to be found.
  img_alt: illustration of a female and male person sitting on large '404'-letters in front of the tum main building
  go_home: Go home
  call_to_action: Give Feedback
</i18n>
