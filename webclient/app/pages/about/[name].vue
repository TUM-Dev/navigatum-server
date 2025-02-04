<script lang="ts" setup>
const route = useRoute();
const { data: page } = await useAsyncData(route.path, () => {
  return queryCollection("content").path(route.path).first();
});
</script>

<template>
  <div id="contentwrapper" class="pt-4">
    <ContentRenderer v-if="page" :value="page" :prose="true" />
  </div>
</template>

<style>
@reference "../../assets/main.css";
#contentwrapper {
  h1 {
    @apply pt-4 pb-3 text-3xl font-medium text-zinc-900;
  }

  h2 {
    @apply pt-3 pb-2 text-2xl font-medium text-zinc-900;
  }

  h3 {
    @apply pt-2 pb-1 text-lg font-medium text-zinc-800;
  }

  p,
  ul,
  ol {
    @apply py-1 text-sm leading-6 font-medium text-zinc-700;
  }

  h4,
  h5,
  h6 {
    @apply py-1 text-sm leading-6 font-medium text-zinc-700;
  }

  li {
    @apply ms-5 list-outside list-disc;
  }

  code {
    @apply mb-4 inline-flex max-w-full items-center space-x-4 overflow-auto rounded-md bg-blue-100 px-4 py-3 text-left font-mono text-xs text-blue-900 text-blue-950 dark:bg-blue-50;
  }

  p a {
    @apply gap-1 rounded-sm bg-transparent text-blue-600 visited:text-blue-600 hover:underline focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus-visible:outline-0;
  }
}
</style>
