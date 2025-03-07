// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false,
  runtimeConfig: {
    public: {
      // Set your backend URL via an environment variable, fallback to localhost
      graphqlRustEndpoint: process.env.GRAPHQL_RUST_ENDPOINT || 'http://localhost:8000/graphql',
      graphqlDjangoEndpoint: process.env.GRAPHQL_DJANGO_ENDPOINT || 'http://localhost:8001/graphql',
    }
  },
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  extends: ['@nuxt/ui-pro'],
  modules: ['@nuxtjs/apollo', '@nuxt/ui'],
  apollo: {
    clients: {
      default: {
        httpEndpoint: process.env.GRAPHQL_RUST_ENDPOINT || 'http://localhost:8000/graphql',
        httpLinkOptions: { useGETForQueries: false },
      },
      DjangoGraphQL: {
        httpEndpoint: process.env.GRAPHQL_DJANGO_ENDPOINT || 'http://localhost:8001/graphql',
        httpLinkOptions: { useGETForQueries: false },
      }
    },
  },
})