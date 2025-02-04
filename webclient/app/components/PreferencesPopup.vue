<script setup lang="ts">
import { Menu, MenuButton, MenuItem, MenuItems } from "@headlessui/vue";
import { AdjustmentsHorizontalIcon } from "@heroicons/vue/24/outline";
import { ComputerDesktopIcon, MoonIcon, SunIcon } from "@heroicons/vue/20/solid";
import SelectionOption from "~/components/SelectionOption.vue";

const colorMode = useColorMode();
const { t } = useI18n({ useScope: "local" });

const { locale } = useI18n();
const switchLocalePath = useSwitchLocalePath();

async function updateLocale(value: "de" | "en") {
  await navigateTo(switchLocalePath(value));
}
</script>

<template>
  <Menu as="div">
    <div>
      <MenuButton
        id="preferences"
        class="focusable ring-opacity-0 hover:ring-opacity-20 focus:ring-opacity-100 relative flex rounded-full bg-transparent p-2 text-sm ring-2 ring-white hover:bg-zinc-100/10 focus:outline-hidden"
      >
        <span class="absolute -inset-1.5" />
        <span class="sr-only">Open preferences menu</span>
        <AdjustmentsHorizontalIcon class="h-6 w-6 text-zinc-900" />
      </MenuButton>
    </div>
    <Transition
      leave-active-class="transition ease-in duration-75"
      leave-from-class="transform opacity-100 scale-100"
      leave-to-class="transform opacity-0 scale-95"
    >
      <MenuItems
        class="ring-opacity-5 absolute top-20 -right-1 z-10 w-48 origin-top-right rounded-xs bg-white py-5 ring-1 shadow-lg ring-black focus:outline-hidden dark:bg-zinc-100"
      >
        <MenuItem as="div" class="block px-4 pb-2 text-xs font-semibold text-zinc-400">
          {{ t("preferences") }}
        </MenuItem>
        <MenuItem as="div" class="text-md block px-4 py-1 font-semibold text-zinc-500">
          <SelectionSwitch v-model="colorMode.preference" label="Theme">
            <SelectionOption value="system">
              <ComputerDesktopIcon class="mt-0.5 h-4 w-4" />
              system
            </SelectionOption>
            <SelectionOption value="dark">
              <MoonIcon class="mb-0.5 h-4 w-4" />
              dark
            </SelectionOption>
            <SelectionOption value="light">
              <SunIcon class="h-4 w-4" />
              light
            </SelectionOption>
          </SelectionSwitch>
        </MenuItem>
        <MenuItem as="div" class="text-md block px-4 py-1 font-semibold text-zinc-500">
          <SelectionSwitch
            v-model="locale"
            :label="t('language')"
            @update:model-value="(value) => updateLocale(value as 'de' | 'en')"
          >
            <SelectionOption value="de">de</SelectionOption>
            <SelectionOption value="en">en</SelectionOption>
          </SelectionSwitch>
        </MenuItem>
      </MenuItems>
    </Transition>
  </Menu>
</template>

<i18n lang="yaml">
de:
  preferences: Präferenzen
  language: Sprache
en:
  preferences: Preferences
  language: Language
</i18n>
