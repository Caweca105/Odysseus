<template>
  <UContainer class="p-4">
    <h1>Rust With REST API</h1>
    <div v-if="loading">Loading users…</div>
    <div v-else-if="error">Error: {{ error.message }}</div>
    <div v-else>
      <div
        class="flex items-center justify-between px-3 py-3.5 border-b border-gray-200 dark:border-gray-700"
      >
        <UInput
          v-model="filter"
          placeholder="Filter users..."
          clearable
          class="max-w-md"
        />
        <!-- Create User Modal -->
        <UModal>
          <UButton label="Create" class="ml-4 rounded-full" />
          <template #content>
            <div class="p-4 mb-4 pb-4 flex flex-col gap-4">
              <h2>Create User</h2>
              <form @submit.prevent="createUser">
                <UFormField label="Username">
                  <UInput
                    placeholder="Enter your username"
                    v-model="newUser.username"
                    type="text"
                    required
                  />
                </UFormField>
                <UFormField label="Email">
                  <UInput
                    placeholder="Enter your email"
                    v-model="newUser.email"
                    type="email"
                    required
                  />
                </UFormField>
                <UFormField label="Age">
                  <UInput
                    placeholder="Enter your age"
                    v-model.number="newUser.age"
                    type="number"
                  />
                </UFormField>
                <UFormField label="Comment">
                  <UInput
                    placeholder="Enter your comment"
                    v-model="newUser.comment"
                    type="text"
                  />
                </UFormField>
                <UFormField label="Location">
                  <UInput
                    placeholder="Enter your location"
                    v-model="newUser.location"
                    type="text"
                  />
                </UFormField>
                <UFormField label="Name">
                  <UInput
                    placeholder="Enter your name"
                    v-model="newUser.name"
                    type="text"
                  />
                </UFormField>
                <UFormField label="Preferences">
                  <UInput
                    placeholder="Enter your preferences"
                    v-model="newUser.preferences"
                    type="text"
                  />
                </UFormField>
                <UButton type="submit" label="Create User" class="mt-4 rounded-full" />
              </form>
            </div>
          </template>
        </UModal>
      </div>

      <!-- Users Table -->
      <UTable :data="rowsWithActions" :columns="columns">
        <template #action-cell="{ row }">
          <div class="flex gap-1 justify-end">
            <UDropdownMenu :items="getDropdownActions(row.original)">
              <UButton
                icon="i-lucide-ellipsis-vertical"
                color="neutral"
                variant="ghost"
              />
            </UDropdownMenu>
            <!-- Edit User Modal -->
            <UModal v-model:open="isEditOpen">
              <template #content>
                <div v-if="selectedUser" class="p-4 mb-4 pb-4 flex flex-col gap-4">
                  <h2>Edit User</h2>
                  <form @submit.prevent="updateUser">
                    <UFormField label="Email">
                      <UInput
                        placeholder="Enter your email"
                        v-model="selectedUser.email"
                        type="email"
                        required
                      />
                    </UFormField>
                    <UFormField label="Age">
                      <UInput
                        placeholder="Enter your age"
                        v-model.number="selectedUser.age"
                        type="number"
                      />
                    </UFormField>
                    <UFormField label="Comment">
                      <UInput
                        placeholder="Enter your comment"
                        v-model="selectedUser.comment"
                        type="text"
                      />
                    </UFormField>
                    <UFormField label="Location">
                      <UInput
                        placeholder="Enter your location"
                        v-model="selectedUser.location"
                        type="text"
                      />
                    </UFormField>
                    <UFormField label="Name">
                      <UInput
                        placeholder="Enter your name"
                        v-model="selectedUser.name"
                        type="text"
                      />
                    </UFormField>
                    <UFormField label="Preferences">
                      <UInput
                        placeholder="Enter your preferences"
                        v-model="selectedUser.preferences"
                        type="text"
                      />
                    </UFormField>
                    <UButton
                      type="submit"
                      label="Update User"
                      class="mt-4 rounded-full"
                    />
                  </form>
                </div>
              </template>
            </UModal>
          </div>
        </template>
      </UTable>
    </div>
  </UContainer>
</template>

<script setup lang="ts">
import { ref, computed, h } from "vue";
import type { TableColumn, DropdownMenuItem } from "@nuxt/ui";

interface User {
  id: number;
  username: string;
  email: string;
  age: number | null;
  comment: string;
  location: string;
  name: string;
  preferences: string;
}

const baseUrl = "http://localhost:8000/api";
const { data, pending: loading, error, refresh } = useFetch(`${baseUrl}/users`);

const isCreateOpen = ref(false);
const isEditOpen = ref(false);
const selectedUser = ref<User | null>(null);

const newUser = ref({
  username: "",
  email: "",
  age: null as number | null,
  comment: "",
  location: "",
  name: "",
  preferences: "",
});

const columns: TableColumn<User>[] = [
  { accessorKey: "id", header: "ID" },
  {
    accessorKey: "name",
    header: "Name",
    cell: ({ row }: { row: { original: User } }) =>
      h("div", { class: "flex items-center gap-3" }, [
        h("div", null, [
          h(
            "p",
            { class: "font-medium text-(--ui-text-highlighted)" },
            row.original.name
          ),
        ]),
      ]),
  },
  { accessorKey: "email", header: "Email" },
  { accessorKey: "age", header: "Age" },
  { accessorKey: "location", header: "Location" },
  { id: "action" },
];

const filter = ref("");

const rowsWithActions = computed(() => {
  const users = (data.value || []) as User[];
  const filtered = !filter.value
    ? users
    : users.filter((user) => {
        const searchTerm = filter.value.toLowerCase();
        return Object.values(user).some((value) =>
          value?.toString().toLowerCase().includes(searchTerm)
        );
      });
  return filtered.map((user) => ({ ...user, actions: "" }));
});

const createUser = async () => {
  try {
    await $fetch(`${baseUrl}/users`, {
      method: "POST",
      body: newUser.value,
      headers: { "Content-Type": "application/json" },
    });
    newUser.value = {
      username: "",
      email: "",
      age: null,
      comment: "",
      location: "",
      name: "",
      preferences: "",
    };
    isCreateOpen.value = false;
    await refresh();
  } catch (err) {
    console.error("Error creating user:", err);
  }
};

const deleteUser = async (id: number) => {
  try {
    await $fetch(`${baseUrl}/users/${id}`, { method: "DELETE" });
    await refresh();
  } catch (err) {
    console.error("Error deleting user:", err);
  }
};

const openModal = (user: User) => {
  selectedUser.value = { ...user };
  isEditOpen.value = true;
};

const updateUser = async () => {
  if (!selectedUser.value) return;
  try {
    await $fetch(`${baseUrl}/users/${selectedUser.value.id}`, {
      method: "PUT",
      body: {
        email: selectedUser.value.email,
        age: selectedUser.value.age,
        comment: selectedUser.value.comment,
        location: selectedUser.value.location,
        name: selectedUser.value.name,
        preferences: selectedUser.value.preferences,
      },
      headers: { "Content-Type": "application/json" },
    });
    isEditOpen.value = false;
    await refresh();
  } catch (err) {
    console.error("Error updating user:", err);
  }
};

function getDropdownActions(user: User): DropdownMenuItem[][] {
  return [
    [
      {
        label: "Edit",
        icon: "i-lucide-edit",
        onSelect: () => {
          selectedUser.value = { ...user };
          isEditOpen.value = true;
        },
      },
      {
        label: "Delete",
        icon: "i-lucide-trash",
        color: "error",
        onSelect: () => {
          if (confirm("Are you sure you want to delete this user?")) {
            deleteUser(user.id);
          }
        },
      },
    ],
  ];
}
</script>
