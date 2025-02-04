<script setup lang="ts">
import type { UseShareOptions } from "@vueuse/core";
import { useClipboard, useShare } from "@vueuse/core";
import type { components } from "~/api_types";
import { ShareIcon } from "@heroicons/vue/16/solid";
import { ClipboardDocumentCheckIcon, ClipboardIcon } from "@heroicons/vue/20/solid";

const props = defineProps<{
  readonly coords: components["schemas"]["Coordinate"];
  readonly name: string;
}>();

const route = useRoute();
const clipboardSource = computed(() => `https://nav.tum.de${route.fullPath}`);
const { t } = useI18n({ useScope: "local" });
const { copy, copied, isSupported: clipboardIsSupported } = useClipboard({ source: clipboardSource });
const { share, isSupported: shareIsSupported } = useShare();

const modalOpen = ref(false);
const shareOptions = () =>
  ({
    title: props.name,
    text: document.title,
    url: clipboardSource.value,
  }) as UseShareOptions;
</script>

<template>
  <button
    type="button"
    :title="t('external_link')"
    :aria-label="t('sharing_options')"
    class="focusable rounded-xs"
    @click="modalOpen = true"
  >
    <ShareIcon class="h-4 w-4 text-blue-600 hover:text-blue-900" />
  </button>
  <ClientOnly>
    <LazyModal v-model="modalOpen" :title="t('share')">
      <div class="flex flex-col gap-5">
        <div class="flex flex-col gap-2">
          <h3 class="text-md font-semibold text-zinc-600">{{ t("open_in") }}</h3>
          <Btn variant="link" :to="`https://www.google.com/maps/search/?api=1&query=${coords.lat}%2C${coords.lon}`"
            >Google Maps
          </Btn>
          <Btn
            variant="link"
            :to="`https://www.openstreetmap.org/?mlat=${coords.lat}&mlon=${coords.lon}#map=17/${coords.lat}/${coords.lon}&layers=T`"
            >OpenStreetMap
          </Btn>
          <Btn variant="link" :to="`geo:${coords.lat},${coords.lon}`">
            {{ t("other_app") }}
          </Btn>
        </div>
        <div class="flex flex-col gap-2">
          <h3 class="text-md font-semibold text-zinc-600">{{ t("share") }}</h3>
          <Btn v-if="shareIsSupported" variant="primary" @click="share(shareOptions)">
            <ShareIcon v-if="copied" class="my-auto h-4 w-4" />
            {{ t("share_link") }}
          </Btn>
          <Btn v-if="clipboardIsSupported" variant="primary" @click="copy()">
            <ClipboardDocumentCheckIcon v-if="copied" class="my-auto h-4 w-4" />
            <ClipboardIcon v-else class="my-auto h-4 w-4" />
            {{ copied ? t("copied") : t("copy_link") }}
          </Btn>
        </div>
      </div>
    </LazyModal>
  </ClientOnly>
</template>

<i18n lang="yaml">
de:
  copied: Kopiert
  copy_link: Link kopieren
  open_in: Öffnen in
  other_app: Andere App ...
  external_link: Externe Links
  sharing_options: Externe Links und optionen diese seite zu teilen
  share: Teilen
  share_link: Teilen mit ...
en:
  copied: Copied
  copy_link: Copy link
  open_in: Open in
  other_app: Other app ...
  external_link: External links
  sharing_options: External links and options to share this page
  share: Share
  share_link: Share with ...
</i18n>
