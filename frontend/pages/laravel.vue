<template>
  <UContainer class="p-4">
    <UCard class="shadow-lg">
      <template #header>
        <UHeading class="text-3xl font-bold">Laravel Framework</UHeading>
      </template>

      <div class="space-y-4">
        <UText>
          Laravel is a popular open-source PHP framework known for its expressive, elegant
          syntax and developer-friendly features such as routing, middleware,
          authentication, and more.
        </UText>

        <UText>
          Created by <strong>Taylor Otwell</strong> in 2011, Laravel aims to simplify
          tasks such as caching, sessions, and routing, allowing developers to build
          robust web applications rapidly.
        </UText>

        <br />

        <UBadge variant="outline" color="info">
          Laravel uses the MVC (Model-View-Controller) architecture pattern, ensuring
          clear separation between logic, UI, and data.
        </UBadge>
      </div>
    </UCard>
  </UContainer>
</template>

<script setup lang="ts"></script>

<style scoped></style>
