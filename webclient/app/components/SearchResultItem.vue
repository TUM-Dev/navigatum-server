<script setup lang="ts">
import { ChevronRightIcon } from "@heroicons/vue/16/solid/index.js";
import type { components } from "~/api_types/index.js";
import PreviewIcon from "~/components/PreviewIcon.vue";
type ResultEntry = components["schemas"]["ResultEntry"];

defineProps<{
  item: ResultEntry;
  highlighted: boolean;
}>();
</script>

<template>
  <div class="flex gap-1 px-2 py-3 md:gap-3 md:px-4" :class="{ 'bg-blue-200': highlighted }">
    <PreviewIcon :item="{ type: item.type, parsed_id: undefined }" />
    <div class="flex flex-col gap-0.5 text-zinc-600">
      <div class="flex flex-col">
        <div
          v-if="(item.type === 'room' || item.type === 'virtual_room' || item.type === 'poi') && item.parsed_id"
          class="flex flex-row items-center"
        >
          <span class="shrink-0 text-zinc-900" v-html="item.parsed_id" />
          <ChevronRightIcon class="h-4 w-4 shrink-0 text-zinc-400" />
          <span class="line-clamp-1 shrink text-sm text-zinc-400" v-html="item.name" />
        </div>
        <span v-else class="line-clamp-1" v-html="item.name" />
      </div>
      <small>
        {{ item.subtext }}
        <template v-if="item.type === 'room' || item.type === 'virtual_room' || item.type === 'poi'"
          >, <b v-html="item.subtext_bold"
        /></template>
      </small>
    </div>
  </div>
</template>
