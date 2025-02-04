<script setup lang="ts">
import { Listbox, ListboxButton, ListboxOptions } from "@headlessui/vue";
import { ChevronUpDownIcon } from "@heroicons/vue/24/outline";

interface Props {
  label?: string;
}

const props = withDefaults(defineProps<Props>(), { label: "" });
const model = defineModel<string>({ required: true });
</script>

<template>
  <div class="flex flex-col">
    <span class="text-sm font-semibold">{{ props.label }}</span>
    <Listbox v-model="model">
      <div class="relative mt-1">
        <ListboxButton
          class="relative w-full cursor-pointer rounded-md border border-zinc-400 bg-zinc-200 py-2 pr-10 pl-3 text-left shadow-md focus:outline-hidden focus-visible:border-blue-500 focus-visible:ring-2 focus-visible:ring-white/75 focus-visible:ring-offset-2 focus-visible:ring-offset-blue-300 sm:text-sm"
        >
          <span class="block truncate text-zinc-600">{{ model }}</span>
          <span class="absolute inset-y-0 right-0 flex items-center pr-2">
            <ChevronUpDownIcon class="h-5 w-5 text-zinc-600" aria-hidden="true" />
          </span>
        </ListboxButton>

        <Transition
          leave-active-class="transition duration-100 ease-in"
          leave-from-class="opacity-100"
          leave-to-class="opacity-0"
        >
          <ListboxOptions
            class="absolute z-30 mt-1 max-h-60 w-full overflow-auto rounded-md bg-zinc-200 py-1 ring-1 shadow-lg ring-black/5 focus:outline-hidden sm:text-sm"
          >
            <slot />
          </ListboxOptions>
        </Transition>
      </div>
    </Listbox>
  </div>
</template>
