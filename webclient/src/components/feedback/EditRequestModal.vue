<script setup lang="ts">
import BasicFeedbackModal from "@/components/feedback/TokenBasedModal.vue";
import { useGlobalStore } from "@/stores/global";

const global = useGlobalStore();
</script>

<template>
  <BasicFeedbackModal :data="global.propose_edits.data">
    <template v-slot:modal>
      <div class="form-group">
        <label class="form-label" for="additional_context">{{ $t("feedback.message") }}</label>
        <textarea
          class="form-input"
          id="additional_context"
          :placeholder="$t('feedback.message')"
          v-model="global.propose_edits.data.additional_context"
          rows="6"
        />
      </div>
      <h2>{{ $t("feedback.coordinates") }}</h2>
      coordinates can be previewed here
    </template>
    <template v-slot:success="{ successUrl }">
      <p>{{ $t("feedback.success.thank_you") }}</p>
      <p>
        {{ $t("feedback.success.response_at") }}
        <a id="feedback-success-url" class="btn-link" :href="successUrl">{{ $t("feedback.success.this_pull_request") }}</a>
      </p>
    </template>
  </BasicFeedbackModal>
</template>

<style lang="scss" scoped>
@import "@/assets/variables";
.modal {
  #additional_context {
    min-width: 100%;
  }
}
</style>
