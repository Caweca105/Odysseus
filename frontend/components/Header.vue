<script setup lang="ts">
const links = [
  {
    label: "Home",
    icon: "i-heroicons-book-open",
    to: "/",
  },
  {
    label: "Rust",
    icon: "i-heroicons-book-open",
    to: "/rust",
    children: [
      { label: "Rust With GraphQL", to: "/rust/rustGraphQL" },
      { label: "Rust with RestAPI", to: "/rust/rustRestAPI" },
      { label: "Rust with gRPC" },
    ],
  },
  {
    label: "Django",
    icon: "i-heroicons-book-open",
    to: "/django",
    children: [
      { label: "Django With GraphQL", to: "/django/djangoGraphQL" },
      { label: "Django with RestAPI" },
      { label: "Django with gRPC" },
    ],
  },
  {
    label: "Laravel",
    icon: "i-heroicons-book-open",
    to: "/laravel",
    children: [
      { label: "Laravel With GraphQL", to: "/laravel/laravelGraphQL" },
      { label: "Laravel with RestAPI" },
      { label: "Laravel with gRPC" },
    ],
  },
];
</script>

<template>
  <UHeader :links="links">
    <template #left>
      <UButton to="/" variant="ghost" color="primary"> Diogo Lemos </UButton>
    </template>

    <template #right>
      <UColorModeSelect class="w-28" />

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
