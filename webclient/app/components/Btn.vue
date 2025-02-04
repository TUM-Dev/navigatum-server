<script setup lang="ts">
export interface Props {
  type?: "submit" | "reset" | "button";
  size?: "sm" | "md" | "lg" | string;
  to?: string | null;
  disabled?: boolean;
  variant?: "action" | "link" | "linkButton" | "info" | "primary" | "secondary" | string;
  ariaLabel?: string;
}

const props = withDefaults(defineProps<Props>(), {
  type: "button",
  size: "md",
  to: "",
  variant: "primary",
  ariaLabel: "",
  disabled: false,
});
const emit = defineEmits(["click"]);
const variantClasses = computed(() => {
  if (props.disabled) {
    switch (props.variant) {
      case "action":
        return "";
      case "primary":
        return "bg-blue-400 text-blue-50";
      case "secondary":
        return "bg-zinc-400 text-zinc-50";
      case "linkButton":
        return "bg-transparent text-blue-600";
      case "link":
        return "bg-transparent text-blue-600 hover:underline";
      default:
        return props.variant;
    }
  }
  switch (props.variant) {
    case "action":
      return "";
    case "primary":
      return "bg-blue-500 visited:text-blue-50 text-blue-50 hover:bg-blue-600 hover:text-white";
    case "secondary":
      return "bg-zinc-500 visited:text-zinc-50 text-zinc-50 hover:bg-zinc-600 hover:text-white";
    case "linkButton":
      return "bg-transparent visited:text-blue-600 text-blue-600 hover:bg-blue-900/10 dark:hover:bg-blue-50/20 hover:text-blue-500";
    case "link":
      return "bg-transparent visited:text-blue-600 text-blue-600 hover:underline";
    default:
      return props.variant;
  }
});
const sizeClasses = computed(() => {
  switch (props.size) {
    case "sm":
      return "text-xs font-semibold px-1.5 rounded-md";
    case "md":
      return "text-md px-4 py-1.5 rounded-xs";
    case "lg":
      return "text-lg px-2.5 rounded-md";
    default:
      return props.size;
  }
});
</script>

<template>
  <NuxtLink
    v-if="!!props.to && !disabled && (props.to.startsWith('http') || props.to.startsWith('geo:'))"
    :to="props.to"
    :aria-label="ariaLabel"
    :type="props.type"
    v-bind="{ disabled: disabled }"
    :class="`focusable flex flex-row gap-1 ${variantClasses} ${sizeClasses} `"
    target="_blank"
    external
  >
    <slot />
  </NuxtLink>
  <NuxtLinkLocale
    v-else-if="!!props.to"
    :to="props.to"
    :aria-label="ariaLabel"
    :type="props.type"
    v-bind="{ disabled: disabled }"
    :class="`focusable flex flex-row gap-1 ${variantClasses} ${sizeClasses}`"
    prefetch-on="interaction"
    @click="emit('click')"
  >
    <slot />
  </NuxtLinkLocale>
  <button
    v-else
    :aria-label="ariaLabel"
    :type="props.type"
    v-bind="{ disabled: disabled }"
    :class="`focusable flex flex-row gap-1 ${variantClasses} ${sizeClasses}`"
    @click="emit('click')"
  >
    <slot />
  </button>
</template>
