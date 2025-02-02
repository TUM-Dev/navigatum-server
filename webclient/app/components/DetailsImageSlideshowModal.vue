<script setup lang="ts">
import "vue3-carousel/dist/carousel.css";
import { Carousel, Navigation, Pagination, Slide } from "vue3-carousel";
import type { components } from "~/api_types";

type ImageInfoResponse = components["schemas"]["ImageInfoResponse"];
const props = defineProps<{ imgs: readonly ImageInfoResponse[] }>();
const { t } = useI18n({ useScope: "local" });
const runtimeConfig = useRuntimeConfig();

const shownImage = defineModel<ImageInfoResponse>("shown_image");
const slideshowOpen = defineModel<boolean>("slideshow_open", {
  required: true,
});

type OnSlideData = {
  currentSlideIndex: number;
  prevSlideIndex: number;
  slidesCount: number;
};

function onSlide(slide: unknown): void {
  // destructured here to make ts happy
  const { currentSlideIndex } = slide as OnSlideData;
  shownImage.value = props.imgs[currentSlideIndex];
}

interface SubTitle {
  title: string;
  url?: string | null;
  text: string;
}

const subtitles = computed<SubTitle[]>(() => {
  if (!shownImage.value) return [];
  return [
    { title: t("source"), ...shownImage.value.source },
    { title: t("license"), ...shownImage.value.license },
    { title: t("author"), ...shownImage.value.author },
  ];
});
</script>

<template>
  <Modal v-model="slideshowOpen" :title="t('header')" class="!min-w-[60vw]">
    <div class="-mx-6 -mt-3">
      <Carousel
        :items-to-show="1.1"
        snap-align="center"
        :autoplay="10_000"
        :pause-autoplay-on-hover="true"
        @slide-end="onSlide"
      >
        <Slide v-for="img in imgs" :key="img.name">
          <div itemscope itemtype="http://schema.org/ImageObject" class="px-2">
            <NuxtImg
              itemprop="contentUrl"
              :alt="t('image_alt')"
              :src="`${runtimeConfig.public.cdnURL}/cdn/lg/${img.name}`"
              class="max-h-2/3 w-full rounded-sm sm:max-h-[30rem]"
            />
            <span v-if="img.license.url" class="hidden" itemprop="license"> {{ img.license.url }}</span>
            <span v-else class="hidden" itemprop="license"> img.license.text</span>
            <span v-if="img.license.url" class="hidden" itemprop="author"> {{ img.author.url }}</span>
            <span v-else class="hidden" itemprop="author"> img.author.text</span>
          </div>
        </Slide>
        <template #addons>
          <Navigation class="" />
          <Pagination />
        </template>
      </Carousel>
    </div>
    <div v-if="shownImage" class="pt-5">
      <div class="grid min-h-20 auto-cols-auto grid-cols-5 gap-5 text-center">
        <div
          v-for="(sub, i) in subtitles"
          :key="i"
          class="text-balance"
          :class="{
            'md:!text-left': i % 3 == 0,
            'md:!text-center': i % 3 == 1,
            'md:!text-right': i % 3 == 2,
            'col-span-5 md:col-span-1': i % 3 != 1,
            'col-span-5 md:col-span-3': i % 3 === 1,
          }"
        >
          <h6 class="text-sm font-semibold text-zinc-600">
            {{ sub.title }}
          </h6>
          <div class="wrap- text-sm text-zinc-600" :class="[i % 3 == 1 ? 'text-xs' : '']">
            <Btn v-if="sub.url" variant="link" size="-ps-1 !inline" :to="sub.url">
              {{ sub.text }}
            </Btn>
            <template v-else>{{ sub.text }}</template>
          </div>
        </div>
      </div>
    </div>
  </Modal>
</template>

<i18n lang="yaml">
de:
  author: Autor
  header: Bilder-Showcase
  image_alt: Ein Bild welches das Gebäude zeigt
  license: Lizenz
  source: Quelle
en:
  author: Author
  header: Image Showcase
  image_alt: Image showing the building
  license: License
  source: Source
</i18n>
