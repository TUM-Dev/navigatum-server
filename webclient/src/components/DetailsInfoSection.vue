<script setup lang="ts">
import { useDetailsStore } from "@/stores/details";
import DetailsImageSlideshowModal from "@/components/DetailsImageSlideshowModal.vue";
import DetailsPropertyTable from "@/components/DetailsPropertyTable.vue";

const state = useDetailsStore();
</script>

<template>
  <!-- Information section (on mobile) -->
  <div class="column col-5 col-sm-12 show-sm mobile-info-section" v-if="state.data?.props?.computed">
    <h2>{{ $t("view_view.info_title") }}</h2>
    <DetailsPropertyTable />
  </div>

  <!-- Informationen card (desktop) -->
  <!-- Some elements are currently duplicate, which is not optimal but should be okay
       as long as only little information is there -->
  <div class="column col-5 col-md-12 hide-sm">
    <div class="card">
      <a
        class="card-image c-hand"
        @click="state.showImageSlideshow(state.image.shown_image_id || 0)"
        v-if="state.image.shown_image"
      >
        <img
          :alt="$t('view_view.header.image_alt')"
          :src="'/cdn/header/' + state.image.shown_image.name"
          class="img-responsive"
          style="width: 100%"
        />
      </a>
      <div class="card-header">
        <div class="card-title h5">{{ $t("view_view.info_title") }}</div>
      </div>
      <div class="card-body">
        <DetailsPropertyTable />
        <div id="maybe-coordinate-inacurate-warning-toast" />
        <div
          class="toast toast-warning"
          v-if="state.data?.type === 'room' && state.data?.maps?.overlays?.default === null"
        >
          {{ $t("view_view.msg.no_floor_overlay") }}
        </div>
        <div class="toast" v-if="state.data?.props?.comment">
          {{ state.data.props.comment }}
        </div>
      </div>
      <!--<div class="card-footer">
          <button class="btn btn-link">Mehr Infos</button>
      </div>-->
    </div>
  </div>
  <DetailsImageSlideshowModal v-if="state.image.slideshow_open" />
</template>

<style lang="scss">
@import "@/assets/variables";
/* --- Information Card (desktop) --- */
.card-body .toast {
  margin-top: 12px;
}
/* --- Information Section (mobile) --- */
.mobile-info-section {
  margin-top: 15px;

  & > .info-table {
    margin-top: 16px;
  }
}
</style>
