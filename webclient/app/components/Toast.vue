<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    id?: string;
    msg?: string;
    level?: "error" | "warning" | "info" | "default";
  }>(),
  { level: "default", msg: "", id: undefined },
);
</script>

<template>
  <div
    v-bind="{ id }"
    :data-cy="'toast-' + props.level"
    class="rounded-sm border p-1.5 text-sm leading-5 text-pretty"
    :class="{
      'border-red-300 bg-red-100 text-red-900': props.level === 'error',
      'border-orange-300 bg-orange-100 text-orange-900': props.level === 'warning',
      'border-blue-300 bg-blue-100 text-blue-900': props.level === 'info',
      'border-zinc-300 bg-zinc-100 text-zinc-900': props.level === 'default',
    }"
  >
    <slot>{{ props.msg }}</slot>
  </div>
</template>
