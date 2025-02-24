// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false,
  runtimeConfig: {
    public: {
      // Set your backend URL via an environment variable, fallback to localhost
      graphqlEndpoint: process.env.GRAPHQL_ENDPOINT || 'http://localhost:8000/graphql'
    }
  },
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  extends: ['@nuxt/ui-pro'],
  modules: ['@nuxtjs/apollo', '@nuxt/ui'],
  apollo: {
    clients: {
      default: {
        // Use the runtime config variable for the endpoint
        httpEndpoint: process.env.GRAPHQL_ENDPOINT || 'http://localhost:8000/graphql',
        // Force Apollo Client to use POST for queries
        httpLinkOptions: {
          useGETForQueries: false,
        },
      }
    },
  },
})