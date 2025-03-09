<script setup lang="ts">
const links = [
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
