<script setup lang="ts">
import { useGlobalStore } from "@/stores/global";
import { useRoute } from "vue-router";

const route = useRoute();
const global = useGlobalStore();
</script>

<template>
  <button
    class="btn btn-link btn-action btn-sm"
    :title="$t('view_view.header.feedback')"
    @click="global.openFeedback('entry', `[${route.params.id}]: `)"
    data-cy="open-feedback-details"
  >
    <i class="icon icon-flag" />
  </button>
</template>
