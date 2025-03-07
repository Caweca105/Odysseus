<script setup lang="ts">
const links = [
  {
    label: "Rust With GraphQL",
    to: "/rustGraphQL",
  },
  {
    label: "Django With GraphQL",
    icon: "i-heroicons-book-open",
    to: "/djangoGraphQL",
  },
];
</script>

<template>
  <UFooter :links="links">
    <template #left> Copyright © {{ new Date().getFullYear() }} </template>

    <template #right>
      <UButton
        icon="i-simple-icons-github"
        color="gray"
        variant="ghost"
        to="https://github.com/Caweca105?tab=repositories"
        target="_blank"
      />
    </template>
  </UFooter>
</template>
