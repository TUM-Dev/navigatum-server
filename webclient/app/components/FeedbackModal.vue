<script setup lang="ts">
import { useFeedback } from "~/composables/feedback";

const { t } = useI18n({ useScope: "local" });
const deleteIssueRequested = ref(false);
const feedback = useFeedback();
</script>

<template>
  <TokenBasedModal :data="feedback.data">
    <template #modal>
      <label class="text-sm font-semibold text-zinc-600" for="feedback-subject"> {{ t("subject") }}</label>
      <div class="flex flex-row gap-2 pb-3 text-zinc-600">
        <select
          id="feedback-category"
          v-model="feedback.data.category"
          class="focusable rounded-sm border border-zinc-400 bg-zinc-200 px-2 py-1"
          :aria-label="t('category')"
        >
          <option value="general">{{ t("type.general") }}</option>
          <option value="bug">{{ t("type.bug") }}</option>
          <option value="feature">{{ t("type.feature") }}</option>
          <option value="search">{{ t("type.search") }}</option>
          <option value="navigation">{{ t("type.navigation") }}</option>
          <option value="entry">{{ t("type.entry") }}</option>
        </select>
        <input
          id="feedback-subject"
          v-model="feedback.data.subject"
          class="focusable grow rounded-sm border border-zinc-400 bg-zinc-200 px-2 py-1"
          type="text"
          :placeholder="t('subject')"
        />
      </div>

      <div class="flex flex-col pb-5">
        <label class="text-sm font-semibold text-zinc-600" for="feedback-body">
          {{ t("message") }}
        </label>
        <textarea
          id="feedback-body"
          v-model="feedback.data.body"
          class="focusable resize-y rounded-sm border border-zinc-400 bg-zinc-200 px-2 py-1"
          :placeholder="t('message')"
          rows="6"
        />
        <p class="text-xs text-zinc-500">{{ t("helptext." + feedback.data.category) }}</p>
      </div>

      <Checkbox id="delete-issue" v-model="deleteIssueRequested">{{ t("delete") }}</Checkbox>
    </template>
    <template #success="{ successUrl }">
      <p>{{ t("success.thank_you") }}</p>
      <I18nT tag="p" keypath="success.response_at">
        <template #this_issue>
          <Btn variant="link" :to="successUrl">{{ t("success.this_issue") }}</Btn>
        </template>
      </I18nT>
    </template>
  </TokenBasedModal>
</template>

<i18n lang="yaml">
de:
  category: Feedback-Kategorie
  delete: Das zugehörige GitHub Issue löschen, sobald es gelöst wurde.
  helptext:
    bug: Welchen Fehler hast du gefunden? Wo hast du ihn gefunden? Bitte gib eine genaue Beschreibung an.
    entry: Feedback zu einem Eintrag. Wir können Räume/Gebäude/Standorte hinzufügen und alle Daten, die du siehst (Namen, Koordinaten, Adressen, ...) anpassen. Was können wir verbessern?
    feature: Features, die du gerne auf dieser Website haben würdest
    general: Generelles Feedback über diese Website
    other: "Feedback ist auf ein Problem gestoßen: Kategorie ungültig"
    search: Feedback zur Suche. Was war dein Suchbegriff? Was hättest du als Ergebnis erwartet?
    navigation: (Beta) Feedback zur Navigation. Von wo bis wo wolltest du navigieren? Welche Probleme hast du gemerkt?
  message: Nachricht
  subject: Betreff
  success:
    response_at: Antwort auf dein Feedback findest du auf {this_issue}
    thank_you: Vielen Dank für dein Feedback! Wir werden es schnellstmöglich bearbeiten.
    this_issue: diesem GitHub Issue
  type:
    bug: Fehler
    entry: Eintrag
    feature: Features
    general: Allgemein
    search: Suche
    navigation: (Beta) Navigation
en:
  category: Feedback category
  delete: Delete this GitHub issue when resolved.
  helptext:
    bug: Which bug did you find? Where did you find it? Please provide a detailed description.
    entry: Feedback about an entry. We can add rooms/buildings/locations and adjust all data you see (names, coordinates, addresses, ...). What can we improve?
    feature: Features you would like to see on this website
    general: General Feedback about this website
    other: "Feedback encountered issue: Category invalid"
    search: Feedback about the search. What was your search query? What did you expect to see?
    navigation: (Beta) Feedback on navigation. From where to where did you want to navigate? What problems did you notice?
  message: Message
  subject: Subject
  success:
    response_at: You can see our response at {this_issue}
    thank_you: Thank you for giving your feedback. We will work on this as soon as possible.
    this_issue: this GitHub issue
  type:
    bug: Bug
    entry: Entry
    feature: Features
    general: General
    search: Search
    navigation: (Beta) Navigation
</i18n>
