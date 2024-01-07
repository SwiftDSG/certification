// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  runtimeConfig: {
    public: {
      apiBase: process.env.API_URL || "http://localhost:8000",
      clientBase: process.env.CLIENT_URL || "http://localhost:3000",
      base: process.env.BASE_URL || "http://localhost:3000",
    },
  },
  app: {
    head: {
      title: "Redian: Content Management System",
      htmlAttrs: {
        lang: "id",
      },
      meta: [
        { charset: "utf-8" },
        { name: "keywords", content: "Website, Application, Digital Agency" },
        {
          name: "viewport",
          content:
            "width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no",
        },
        { name: "format-detection", content: "telephone=no" },
        { name: "googlebot", content: "notranslate" },
      ],
      link: [
        {
          rel: "icon",
          href: `${
            process.env.BASE_URL || "http://localhost:3000"
          }/favicon.svg`,
          type: "image/svg+xml",
        },
        {
          rel: "preconnect",
          href: process.env.API_URL,
          crossorigin: "use-credentials",
        },
      ],
      script: [
        { src: `${process.env.BASE_URL || "http://localhost:3000"}/config.js` },
      ],
    },
  },
  css: ["~/assets/fonts.scss"],
});
