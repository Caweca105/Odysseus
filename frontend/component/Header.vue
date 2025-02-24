<script setup lang="ts">
const links = [
  {
    label: "Home",
    icon: "i-heroicons-book-open",
    to: "/",
  },
  {
    label: "About",
    icon: "i-heroicons-book-open",
    to: "/about",
  },
  {
    label: "Projects",
    icon: "i-heroicons-square-3-stack-3d",
    to: "/projects",
  },
  {
    label: "Planning",
    icon: "i-heroicons-rocket-launch",
    to: "/planning",
  },
  {
    label: "Contact",
    icon: "i-heroicons-rocket-launch",
    to: "/contact",
  },
  {
    label: "Blog",
    icon: "i-heroicons-rocket-launch",
    to: "/blog",
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
