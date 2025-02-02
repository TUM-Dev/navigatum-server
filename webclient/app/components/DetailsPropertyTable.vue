<script setup lang="ts">
import { ArrowTopRightOnSquareIcon, InformationCircleIcon } from "@heroicons/vue/24/outline";
import type { components } from "~/api_types";

type Props = components["schemas"]["PropsResponse"];

defineProps<{ props: Props; id: string; name: string; navigationEnabled: boolean }>();
const { t } = useI18n({ useScope: "local" });
</script>

<template>
  <div v-if="props.links || props.computed || navigationEnabled" class="flex flex-col gap-3 text-zinc-800">
    <p v-for="prop in props.computed" :key="prop.name" class="flex flex-col">
      <span class="text-xs font-semibold text-zinc-500 uppercase">{{ prop.name }}</span>
      <span>{{ prop.text }}</span>
      <TinyModal v-if="prop.extra?.body" :content="prop.extra">
        <template #icon>
          <InformationCircleIcon class="h-4 w-4" />
        </template>
      </TinyModal>
    </p>
    <ul v-if="navigationEnabled || props.links" class="flex flex-col gap-1.5">
      <li v-if="navigationEnabled" class="print:!hidden">
        <Btn
          size="text-md gap-2.5 px-3 py-1.5 rounded-sm leading-snug"
          variant="secondary"
          :to="`/navigate?coming_from=${id}&to=${id}&q_to=${name}`"
        >
          <span
            class="my-auto me-2 h-5 min-h-5 min-w-5 rounded-sm bg-blue-100 px-2.5 py-0.5 text-xs font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-300"
            >BETA</span
          >
          {{ t("start navigation") }}
        </Btn>
      </li>
      <li v-for="link in props.links" :key="link.text">
        <Btn
          size="text-md gap-2.5 px-3 py-1.5 rounded-sm leading-snug print:!text-blue-500"
          variant="secondary"
          :to="link.url"
        >
          <ArrowTopRightOnSquareIcon class="my-auto h-5 min-h-5 w-5 min-w-5 pb-0.5" />
          {{ link.text }}
        </Btn>
      </li>
    </ul>
  </div>
  <div v-else>
    {{ t("no_information_known") }}
  </div>
</template>

<i18n lang="yaml">
de:
  links: Links
  no_information_known: Keine Informationen bekannt
  start navigation: Navigation starten
en:
  links: Links
  no_information_known: No information known
  navigate from here: Start navigation
</i18n>
