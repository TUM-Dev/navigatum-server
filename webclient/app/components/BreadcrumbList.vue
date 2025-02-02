<script setup lang="ts">
type Item = { to: string; name: string };
const props = withDefaults(defineProps<{ items: Item[]; class?: string }>(), { class: "" });
</script>

<template>
  <ol
    vocab="https://schema.org/"
    typeof="BreadcrumbList"
    class="flex flex-row flex-wrap gap-2 text-sm"
    :class="props.class"
  >
    <template v-for="(item, i) in props.items" :key="item.to">
      <span v-if="i > 0" aria-hidden="true" class="text-zinc-500">/</span>
      <li property="itemListElement" typeof="ListItem">
        <NuxtLinkLocale
          :to="item.to"
          property="item"
          typeof="WebPage"
          class="focusable rounded-xs hover:underline"
          :class="{
            'visited:text-blue-500': i > 0,
            'visited:text-zinc-500': i === 0,
            'hover:text-blue-600': i > 0,
            'hover:text-zinc-600': i === 0,
            'text-blue-500': i > 0,
            'text-zinc-500': i === 0,
          }"
        >
          <span property="name">{{ item.name }}</span>
        </NuxtLinkLocale>
        <meta property="position" :content="`${i + 1}`" />
      </li>
    </template>
  </ol>
</template>
