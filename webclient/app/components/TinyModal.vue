<script setup lang="ts">
import { useBreakpoints } from "@vueuse/core";

const props = defineProps<{
  content: {
    title?: string | null;
    body: string;
    footer?: string | null;
  };
}>();

const { t } = useI18n({ useScope: "local" });
const breakpoints = useBreakpoints({
  xs: 0,
  sm: 601,
  md: 841,
  lg: 961,
  xl: 1281,
});
const showPopoverInstead = breakpoints.greaterOrEqual("md");
const modalOpen = ref(false);
</script>

<template>
  <div v-if="showPopoverInstead" class="popover">
    <slot name="icon" />
    <div class="popover-container">
      <div class="rounded-sm shadow">
        <div v-if="props.content.title" class="card-header">
          {{ props.content.title }}
        </div>
        <div class="card-body">
          {{ props.content.body }}
        </div>
        <div v-if="props.content.footer" class="card-footer">
          {{ props.content.footer }}
        </div>
      </div>
    </div>
  </div>
  <template v-else>
    <a class="cursor-pointer" :aria-label="t('show_more_information')" @click="() => (modalOpen = true)">
      <slot name="icon" />
    </a>
    <ClientOnly>
      <LazyModal v-model="modalOpen" :title="props.content.title || ''">
        <p v-if="props.content.body">{{ props.content.body }}</p>
        <p v-if="props.content.footer">{{ props.content.footer }}</p>
      </LazyModal>
    </ClientOnly>
  </template>
</template>

<i18n lang="yaml">
de:
  show_more_information: Mehr Informationen anzeigen
en:
  show_more_information: Show more information
</i18n>
