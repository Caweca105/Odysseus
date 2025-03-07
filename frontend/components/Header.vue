<script setup lang="ts">
const links = [
  {
    label: "Home",
    icon: "i-heroicons-book-open",
    to: "/",
  },
  {
    label: "Rust With GraphQL",
    icon: "i-heroicons-book-open",
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
  <UHeader :links="links">
    <template #left>
      <UButton to="/" variant="ghost" color="primary"> Diogo Lemos </UButton>
    </template>

    <template #right>
      <UTooltip text="Change my theme">
        <UColorModeButton />
      </UTooltip>

      <UTooltip text="This will take you to my repositories on GitHub">
        <UButton
          to="https://github.com/Caweca105?tab=repositories"
          target="_blank"
          icon="i-simple-icons-github"
          color="primary"
          variant="ghost"
        />
      </UTooltip>
    </template>
  </UHeader>
</template>
