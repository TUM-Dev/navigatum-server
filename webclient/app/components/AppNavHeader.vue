<template>
  <header class="fixed top-0 z-20 w-full bg-zinc-50 shadow-md shadow-black/5 dark:bg-zinc-100 dark:shadow-black/20">
    <nav class="mx-auto max-w-4xl">
      <div class="mx-3 my-2.5 flex place-content-between content-between justify-between gap-x-3 md:gap-x-10 lg:mx-10">
        <NuxtLinkLocale class="my-auto hidden lg:block print:!block" to="/">
          <img alt="Navigatum Logo" class="pt-2" width="201" height="32" src="~/assets/logos/navigatum.svg" />
        </NuxtLinkLocale>
        <div class="my-auto max-w-xl grow lg:col-span-3 print:!hidden">
          <slot />
        </div>
        <div class="relative my-auto shrink-0 xl:absolute xl:top-3 xl:right-5 print:!hidden">
          <PreferencesPopup />
        </div>
      </div>
    </nav>
  </header>
</template>
