// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false,
  css: ['~/assets/css/main.css'],
  runtimeConfig: {
    public: {
      graphqlRustEndpoint: process.env.GRAPHQL_RUST_ENDPOINT || 'http://localhost:8000/graphql',
      graphqlDjangoEndpoint: process.env.GRAPHQL_DJANGO_ENDPOINT || 'http://localhost:8001/graphql',
      graphqlLaravelEndpoint: process.env.GRAPHQL_LARAVEL_ENDPOINT || 'http://localhost:8002/graphql',
      restRustEndpoint: process.env.REST_RUST_ENDPOINT || 'http://localhost:8000/api',
      restDjangoEndpoint: process.env.REST_DJANGO_ENDPOINT || 'http://localhost:8001/api/',
      restLaravelEndpoint: process.env.REST_LARAVEL_ENDPOINT || 'http://localhost:8002/api',
    }
  },
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  extends: [],
  modules: ['@nuxtjs/apollo', '@nuxt/ui', '@nuxt/ui-pro'],
  apollo: {
    clients: {
      RustGraphQL: {
        httpEndpoint: process.env.GRAPHQL_RUST_ENDPOINT || 'http://localhost:8000/graphql',
        httpLinkOptions: { useGETForQueries: false },
      },
      DjangoGraphQL: {
        httpEndpoint: process.env.GRAPHQL_DJANGO_ENDPOINT || 'http://localhost:8001/graphql',
        httpLinkOptions: { useGETForQueries: false },
      },
      LaravelGraphQL: {
        httpEndpoint: process.env.GRAPHQL_LARAVEL_ENDPOINT || 'http://localhost:8002/graphql',
        httpLinkOptions: { useGETForQueries: false },
      }
    },
  },
})