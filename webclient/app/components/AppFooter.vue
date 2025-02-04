<script setup lang="ts">
import { useFeedback } from "~/composables/feedback";

const props = defineProps<{ class?: string }>();
const feedback = useFeedback();
// If we do not include the image here like this, vite/rollup is unable to load it
const colorMode = useColorMode();
const { t, locale } = useI18n({ useScope: "local" });
const navigation = computed(() => [
  {
    name: t("sourcecode.text"),
    href: "https://github.com/TUM-Dev/navigatum",
  },
  {
    name: t("api.text"),
    href: "https://nav.tum.de/api",
  },
  {
    name: t("about.text"),
    href: t("about.link"),
  },
  {
    name: t("privacy.text"),
    href: t("privacy.link"),
  },
  {
    name: t("imprint.text"),
    href: t("imprint.link"),
  },
]);
</script>

<template>
  <footer data-cy="main-footer" class="bg-zinc-100 print:!hidden" :class="props.class">
    <div class="mx-auto max-w-7xl overflow-hidden px-6 py-20 sm:py-14 lg:px-8">
      <nav class="-mb-6 columns-2 text-center sm:columns-3 sm:justify-center sm:space-x-12 md:flex" aria-label="Footer">
        <div v-for="item in navigation" :key="item.name" class="pb-6 text-sm leading-6">
          <Btn variant="link" :to="item.href">{{ item.name }}</Btn>
        </div>
        <div class="pb-6 text-sm leading-6">
          <Btn
            variant="link"
            :aria-label="t('feedback.open')"
            @click="
              () => {
                feedback.open = true;
                feedback.data = {
                  category: 'general',
                  subject: '',
                  body: '',
                  deletion_requested: false,
                };
              }
            "
          >
            {{ t("feedback.text") }}
          </Btn>
        </div>
      </nav>
      <div class="mt-10 flex justify-center space-x-10 text-center">
        <Btn to="https://tum.de" variant="rounded-xl pt-2 pb-4 px-4 focusable" size="sm">
          <p class="text-center text-xs text-zinc-600">
            {{ t("official_roomfinder") }}<br />
            <img
              :alt="t('tum_logo_alt')"
              :src="`/logos/tum_${colorMode.value === 'dark' ? 'dark' : 'light'}_${locale}.svg`"
              width="200"
              height="80"
              loading="lazy"
              aria-hidden="true"
            />
          </p>
        </Btn>
      </div>
    </div>
  </footer>
</template>

<i18n lang="yaml">
de:
  about:
    link: /about/ueber-uns
    text: Über uns
  api:
    link: /api
    text: API
  feedback:
    open: Feedback Form öffnen
    text: Feedback senden
  imprint:
    link: /about/impressum
    text: Impressum
  language: Sprache
  official_roomfinder: Offizieller Roomfinder
  privacy:
    link: /about/datenschutz
    text: Datenschutz
  sourcecode:
    text: Source Code
  theme: Theme
  tum_logo_alt: The Logo of the Technical University Munich
en:
  about:
    link: /en/about/about-us
    text: About us
  api:
    link: /en/api
    text: API
  feedback:
    open: Open the feedback-form
    text: Feedback
  imprint:
    link: /en/about/imprint
    text: Imprint
  language: Language
  official_roomfinder: Official roomfinder
  privacy:
    link: /en/about/privacy
    text: Privacy
  sourcecode:
    text: Source Code
  theme: Theme
  tum_logo_alt: Das Logo der Technischen Universität München
</i18n>
