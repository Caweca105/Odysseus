<template>
  <UContainer class="p-4">
    <UCard>
      <UHeading class="text-3xl font-bold">Rust</UHeading>
      <div class="space-y-4 pt-4">
        <br />

        <p>
          Rust is a systems programming language that runs
          <strong>blazingly fast</strong>, prevents segfaults, and guarantees thread
          safety.
        </p>

        <p>
          Rust is a multi-paradigm programming language focused on performance and safety,
          especially safe concurrency. Rust is syntactically similar to C++, but can
          guarantee memory safety by using a <strong>borrow checker</strong> to validate
          references. Rust achieves memory safety without garbage collection, and
          reference counting is optional.
        </p>

        <p>
          Rust was originally designed by <strong>Graydon Hoare</strong> at Mozilla
          Research, with contributions from Dave Herman, Brendan Eich, and others. The
          designers refined the language while writing the <strong>Servo</strong> layout
          or browser engine, and the Rust compiler. The compiler is free and open-source
          software dual-licensed under the <strong>MIT License</strong> and
          <strong>Apache License 2.0</strong>.
        </p>

        <br />

        <UBadge variant="outline" color="success">
          Rust has been the <strong>"most loved programming language"</strong> in the
          Stack Overflow Developer Survey every year since 2016..
        </UBadge>

        <br />

        <p>
          Rust is syntactically similar to C++, but its designers intend it to provide
          better memory safety while still maintaining performance. Rust is a systems
          programming language that is focused on three goals:
          <strong>safety, speed, and concurrency</strong>. It maintains these goals
          without having a garbage collector, making it a useful language for a number of
          use cases other languages aren’t good at: embedding in other languages, programs
          with specific space and time requirements, and writing low-level code, like
          device drivers and operating systems.
        </p>
        <br />
        <UBadge variant="outline" color="error">
          Rust is designed to be memory safe, and it does not permit null or dangling
          pointers.
        </UBadge>
      </div>
    </UCard>
  </UContainer>
</template>

<script setup lang="ts"></script>

<style scoped></style>
