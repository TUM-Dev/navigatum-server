import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-10-29",
  future: {
    compatibilityVersion: 4,
  },
  runtimeConfig: {
    public: {
      apiURL: "https://nav.tum.de",
      cdnURL: "https://nav.tum.de",
      feedbackURL: "https://nav.tum.de",
    },
  },
  modules: ["@nuxt/eslint", "@nuxtjs/i18n", "@nuxtjs/color-mode", "@vueuse/nuxt", "@nuxt/content", "@nuxt/image"],
  app: {
    head: {
      bodyAttrs: { class: "bg-zinc-50" },
      charset: "utf-8",
      viewport: "width=device-width, initial-scale=1",
      link: [
        {
          rel: "search",
          title: "searchTitle",
          type: "application/opensearchdescription+xml",
          href: "/opensearchdescription.xml",
        },
        { rel: "apple-touch-icon", sizes: "180x180", href: "/favicons/apple-touch-icon.png" },
        { rel: "icon", type: "image/png", sizes: "32x32", href: "/favicons/favicon-32x32.png" },
        { rel: "icon", type: "image/png", sizes: "16x16", href: "/favicons/favicon-16x16.png" },
        { rel: "manifest", href: "/site.webmanifest" },
        { rel: "mask-icon", href: "/favicons/safari-pinned-tab.svg", color: "#0065bd" },
      ],
      meta: [
        { name: "msapplication-TileColor", content: "#0065bd" },
        { name: "theme-color", content: "#ffffff" },
        { name: "author", content: "OpenSource @ TUM e.V. partnering with TUM IT Management" },
        { name: "copyright", content: "GNU General Public License v3.0. Images may be licensed differently." },
        { name: "robots", content: "index, follow" },
        { name: "rating", content: "safe for kids" },
        { name: "og:url", content: "https://nav.tum.de" },
        { name: "og:image:alt", content: "Navigatum Logo" },
        { name: "og:image:width", content: "1200" },
        { name: "og:image:height", content: "630" },
        { name: "og:image:mime", content: "image/png" },
        { name: "og:site_name", content: "NavigaTUM" },
      ],
      script: [
        {
          innerHTML: `{
"@context": "https://schema.org",
"@type": "WebSite",
"url": "https://nav.tum.de/",
"potentialAction": [
  {
    "@type": "SearchAction",
    "target": {
      "@type": "EntryPoint",
      "urlTemplate": "https://nav.tum.de/search?q={search_term_string}"
    },
    "query-input": "required name=search_term_string"
  }
]
}`,
          type: "application/ld+json",
        },
        {
          innerHTML: `{
"@context": "https://schema.org",
"@type": "Organization",
"url": "https://nav.tum.de/",
"logo": "https://nav.tum.de/logos/org_logo.svg"
}`,
          type: "application/ld+json",
        },
        { innerHTML: "window.$plausible = [];" },
      ],
    },
  },
  i18n: {
    baseUrl: "https://nav.tum.de",
    strategy: "prefix_except_default",
    locales: [
      {
        code: "en",
        language: "en-US",
      },
      {
        code: "de",
        language: "de-DE",
      },
    ],
    defaultLocale: "de",
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: "lang",
      cookieCrossOrigin: false,
      redirectOn: "root", // only redirect if somebody visits / to have better SEO
    },
  },
  css: ["~/assets/main.scss"],
  vite: {
    plugins: [tailwindcss()],
  },
  css: ["~/assets/main.scss"],
  vite: {
    plugins: [tailwindcss()],
  },
  devtools: { enabled: true },
  colorMode: {
    classSuffix: "",
    storageKey: "theme",
    preference: "system",
    fallback: "light",
  },
  content: {},
  nitro: {
    compressPublicAssets: {
      gzip: true,
      brotli: true,
    },
  },
  routeRules: {
    "/": { prerender: true },
    "/about/**": { prerender: true },
    "/en/about/**": { prerender: true },
    "/en/api": { prerender: true },
    "/api": { prerender: true },
    "/view/**": { swr: 3600 },
    "/campus/**": { swr: 3600 },
    "/site/**": { swr: 3600 },
    "/building/**": { swr: 3600 },
    "/room/**": { swr: 3600 },
    "/poi/**": { swr: 3600 },
    "/en/view/**": { swr: 3600 },
    "/en/campus/**": { swr: 3600 },
    "/en/site/**": { swr: 3600 },
    "/en/building/**": { swr: 3600 },
    "/en/room/**": { swr: 3600 },
    "/en/poi/**": { swr: 3600 },
    "/next": { ssr: false },
    "/en/next": { ssr: false },
  },
  typescript: {
    typeCheck: false, // we already typecheck in CI => no need to
    strict: true,
  },
  sourcemap: {
    server: false,
    client: true,
  },
  image: {
    domains: ["nav.tum.de"],
  },
  build: {
    transpile: [
      "sharp", // sharp somehow has problems when not transpiled causing "module not transpiled" errors
    ],
  },
  eslint: {},
});
