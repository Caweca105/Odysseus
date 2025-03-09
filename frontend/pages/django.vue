<template>
  <UContainer class="p-4">
    <UCard>
      <template #header>
        <UHeading class="text-3xl font-bold">Django</UHeading>
      </template>
      <div class="space-y-4">
        <UText>
          Django is a high-level Python web framework that encourages rapid development
          and clean, pragmatic design. It's known for its powerful ORM, robust admin
          interface, and built-in security features.
        </UText>

        <UText>
          Initially developed by <strong>Adrian Holovaty and Simon Willison</strong> at
          Lawrence Journal-World in 2003, Django follows the
          <strong>"batteries-included"</strong> philosophy, offering tools for almost
          every common web development task.
        </UText>

        <UBadge color="red" variant="outline">
          Django prioritizes security with built-in protections against SQL injection,
          cross-site scripting (XSS), cross-site request forgery (CSRF), and clickjacking.
        </UBadge>
      </div>
    </UCard>
  </UContainer>
</template>

<script setup lang="ts"></script>

<style scoped></style>
