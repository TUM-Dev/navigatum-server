<script setup lang="ts">
import { CheckIcon } from "@heroicons/vue/24/outline";
import { ListboxOption } from "@headlessui/vue";

defineProps<{ value: string }>();
</script>

<template>
  <ListboxOption v-slot="{ active, selected }" :value="value" as="template">
    <li
      class="relative cursor-pointer py-2 pr-4 pl-4 select-none"
      :class="[active ? 'bg-blue-100 text-blue-900' : 'text-zinc-900']"
    >
      <span class="flex flex-row gap-1.5 truncate text-sm" :class="[selected ? 'font-bold' : 'font-normal']">
        <slot />
      </span>
      <span v-if="selected" class="absolute inset-y-0 right-0 flex items-center pr-3 text-blue-600">
        <CheckIcon class="h-5 w-5" aria-hidden="true" />
      </span>
    </li>
  </ListboxOption>
</template>
