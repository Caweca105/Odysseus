<script setup lang="ts">
const route = useRoute();

const items = computed(() => [
  {
    label: "Home",
    to: "/",
    active: route.path === "/",
  },
  {
    label: "Rust",
    to: "/rust",
    children: [
      { label: "Rust With GraphQL", to: "/rust/rustGraphQL" },
      { label: "Rust with RestAPI", to: "/rust/rustRestAPI" },
      { label: "Rust with gRPC" },
    ],
    active: route.path.startsWith("/rust"),
  },
  {
    label: "Django",
    to: "/django",
    children: [
      { label: "Django With GraphQL", to: "/django/djangoGraphQL" },
      { label: "Django with RestAPI" },
      { label: "Django with gRPC" },
    ],
    active: route.path.startsWith("/django"),
  },
  {
    label: "Laravel",
    to: "/laravel",
    children: [
      { label: "Laravel With GraphQL", to: "/laravel/laravelGraphQL" },
      { label: "Laravel with RestAPI" },
      { label: "Laravel with gRPC" },
    ],
    active: route.path.startsWith("/laravel"),
  },
]);
</script>

<template>
  <UHeader>
    <template #left>
      <UButton to="/" variant="ghost" color="primary"> Diogo Lemos </UButton>
    </template>

    <UNavigationMenu :items="items" />

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
