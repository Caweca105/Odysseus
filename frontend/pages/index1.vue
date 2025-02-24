<script setup lang="ts">
import { useQuery } from "@vue/apollo-composable";
import gql from "graphql-tag";

const GET_HELLO = gql`
  query {
    hello
  }
`;

// Perform the async query (suitable for SSR)
const { result: data, loading, error } = useQuery(GET_HELLO);

console.log("GraphQL Endpoint:", useRuntimeConfig().public.graphqlEndpoint);
</script>

<template>
  <div>
    <h1>Nuxt + Rust GraphQL</h1>
    <p v-if="loading">Loading…</p>
    <p v-else-if="error">Error: {{ error.message }}</p>
    <p v-else>Backend says: {{ data?.hello }}</p>
  </div>
</template>
