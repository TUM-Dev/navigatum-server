<script setup lang="ts">
defineProps<{ id: string }>();
const model = defineModel<boolean>();
</script>

<template>
  <div class="mb-4 flex gap-2">
    <div class="flex h-5 min-w-4 items-center rounded-sm">
      <input
        :id="id"
        v-model="model"
        type="checkbox"
        class="focusable z-40 min-h-4 min-w-4 border-zinc-300 bg-zinc-100 accent-blue-400"
      />
    </div>
    <div class="ps-1">
      <label :for="id" class="z-30 text-sm font-medium text-zinc-900">
        <slot />
      </label>
      <p v-if="$slots.helptext" class="text-xs font-normal text-zinc-500">
        <slot name="helptext" />
      </p>
    </div>
  </div>
</template>
