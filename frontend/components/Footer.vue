<script setup lang="ts">
const items = [
  {
    label: "Rust",
    to: "/rust",
  },
  {
    label: "Django",
    to: "/django",
  },
  {
    label: "Laravel",
    to: "/laravel",
  },
];
</script>

<template>
  <UFooter>
    <template #left> Copyright © {{ new Date().getFullYear() }} </template>

    <UNavigationMenu :items="items" variant="link" />

    <template #right>
      <UButton
        icon="i-simple-icons-github"
        color="neutral"
        variant="ghost"
        to="https://github.com/Caweca105?tab=repositories"
        target="_blank"
      />
    </template>
  </UFooter>
</template>
