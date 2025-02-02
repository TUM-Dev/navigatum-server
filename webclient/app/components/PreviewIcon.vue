<script setup lang="ts">
import {
  BuildingOffice2Icon,
  BuildingOfficeIcon,
  MagnifyingGlassIcon,
  MapPinIcon,
} from "@heroicons/vue/24/outline/index.js";
import type { components } from "~/api_types/index.js";

type ResultEntry = components["schemas"]["ResultEntry"];
type ResultEntryItem = {
  type: ResultEntry["type"];
  parsed_id: ResultEntry["parsed_id"];
};
defineProps<{ item: ResultEntryItem }>();
</script>

<template>
  <div class="my-auto min-w-9 md:min-w-11">
    <div v-if="item.type === 'room' || item.type === 'virtual_room' || item.type === 'poi'" class="p-2 text-zinc-900">
      <MagnifyingGlassIcon v-if="item.parsed_id" class="h-5 w-5 md:h-6 md:w-6" />
      <MapPinIcon v-else class="h-5 w-5 md:h-6 md:w-6" />
    </div>
    <div v-else class="rounded-full bg-blue-500 p-2 text-white">
      <BuildingOfficeIcon v-if="item.type === 'building'" class="mx-auto h-5 w-5 md:h-6 md:w-6" />
      <BuildingOffice2Icon v-else class="mx-auto h-5 w-5 md:h-6 md:w-6" />
    </div>
  </div>
</template>
