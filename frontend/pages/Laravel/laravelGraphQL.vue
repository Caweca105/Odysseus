<template>
  <UContainer class="p-4">
    <h1>Laravel with GraphQL</h1>

    <!-- Loading and Error States -->
    <div v-if="loading">Loading users…</div>
    <div v-else-if="error">Error: {{ error.message }}</div>
    <div v-else>
      <!-- Filter + Create Button -->
      <div
        class="flex items-center justify-between px-3 py-3.5 border-b border-gray-200 dark:border-gray-700"
      >
        <UInput
          v-model="filter"
          placeholder="Filter users..."
          clearable
          class="max-w-md"
        />
        <UButton
          label="Create"
          @click="isCreateOpen = true"
          :ui="{ rounded: 'rounded-full' }"
          class="ml-4"
        />
      </div>

      <!-- Table of Users -->
      <UTable :rows="rowsWithActions" :columns="columns">
        <template #actions-data="{ row }">
          <div class="flex gap-1 justify-end">
            <UButton
              @click="openModal(row)"
              :ui="{ rounded: 'rounded-full' }"
              icon="i-heroicons-pencil-square"
            />
            <UButton
              @click="deleteUser(row.id)"
              :ui="{ rounded: 'rounded-full' }"
              icon="i-heroicons-trash"
            />
          </div>
        </template>
      </UTable>

      <!-- Create User Modal -->
      <UModal v-model="isCreateOpen">
        <div class="p-4 mb-4 pb-4 flex flex-col gap-4">
          <h2>Create User</h2>
          <form @submit.prevent="createUser">
            <div>
              <label>Username:</label>
              <UInput v-model="newUser.username" type="text" required />
            </div>
            <div>
              <label>Email:</label>
              <UInput v-model="newUser.email" type="email" required />
            </div>
            <div>
              <label>Age:</label>
              <UInput v-model.number="newUser.age" type="number" />
            </div>
            <div>
              <label>Comment:</label>
              <UInput v-model="newUser.comment" type="text" />
            </div>
            <div>
              <label>Location:</label>
              <UInput v-model="newUser.location" type="text" />
            </div>
            <div>
              <label>Name:</label>
              <UInput v-model="newUser.name" type="text" />
            </div>
            <div>
              <label>Preferences:</label>
              <UInput v-model="newUser.preferences" type="text" />
            </div>
            <UButton
              type="submit"
              label="Create User"
              :ui="{ rounded: 'rounded-full' }"
              class="mt-4"
            />
          </form>
        </div>
      </UModal>

      <!-- Edit User Modal -->
      <UModal v-model="isEditOpen">
        <div v-if="selectedUser" class="p-4 mb-4 pb-4 flex flex-col gap-4">
          <h2>Edit User</h2>
          <form @submit.prevent="updateUser">
            <div>
              <label>Email:</label>
              <UInput v-model="selectedUser.email" type="email" required />
            </div>
            <div>
              <label>Age:</label>
              <UInput v-model.number="selectedUser.age" type="number" />
            </div>
            <div>
              <label>Comment:</label>
              <UInput v-model="selectedUser.comment" type="text" />
            </div>
            <div>
              <label>Location:</label>
              <UInput v-model="selectedUser.location" type="text" />
            </div>
            <div>
              <label>Name:</label>
              <UInput v-model="selectedUser.name" type="text" />
            </div>
            <div>
              <label>Preferences:</label>
              <UInput v-model="selectedUser.preferences" type="text" />
            </div>
            <UButton
              type="submit"
              label="Update User"
              :ui="{ rounded: 'rounded-full' }"
              class="mt-4"
            />
          </form>
        </div>
      </UModal>
    </div>
  </UContainer>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useQuery, useMutation } from "@vue/apollo-composable";
import gql from "graphql-tag";

/**
 * Modals and selected user state
 */
const isCreateOpen = ref(false);
const isEditOpen = ref(false);
const selectedUser = ref<any>(null);

/**
 * GQL: Queries & Mutations (Laravel backend)
 */
const GET_USERS = gql`
  query {
    users {
      id
      username
      email
      age
      comment
      location
      name
      preferences
    }
  }
`;

const CREATE_USER = gql`
  mutation CreateUser($input: NewUserInput!) {
    createUser(input: $input) {
      id
      username
      email
      age
      comment
      location
      name
      preferences
    }
  }
`;

const UPDATE_USER = gql`
  mutation UpdateUser($input: UpdateUserInput!) {
    updateUser(input: $input) {
      id
      username
      email
      age
      comment
      location
      name
      preferences
    }
  }
`;

const DELETE_USER = gql`
  mutation DeleteUser($id: ID!) {
    deleteUser(id: $id)
  }
`;

/**
 * Apollo Queries/Mutations
 */
const { result, loading, error, refetch } = useQuery(GET_USERS, null, {
  clientId: "LaravelGraphQL",
});

const { mutate: createUserMutate } = useMutation(CREATE_USER, {
  clientId: "LaravelGraphQL",
});
const { mutate: updateUserMutate } = useMutation(UPDATE_USER, {
  clientId: "LaravelGraphQL",
});
const { mutate: deleteUserMutate } = useMutation(DELETE_USER, {
  clientId: "LaravelGraphQL",
});

/**
 * New user form
 */
const newUser = ref({
  username: "",
  email: "",
  age: null as number | null,
  comment: "",
  location: "",
  name: "",
  preferences: "",
});

/**
 * Table columns + row filtering
 */
const columns = [
  { key: "username", label: "Username" },
  { key: "email", label: "Email" },
  { key: "age", label: "Age" },
  { key: "location", label: "Location" },
  { key: "name", label: "Name" },
  { key: "actions", label: "Actions" },
];

const filter = ref("");

/** Hide empty placeholders. */
function isEmptyUser(user: any) {
  const isEmptyField = (field: string | null) => !field || field.trim() === "";
  return (
    isEmptyField(user.username) &&
    isEmptyField(user.email) &&
    user.age === null &&
    isEmptyField(user.comment) &&
    isEmptyField(user.location) &&
    isEmptyField(user.name) &&
    isEmptyField(user.preferences)
  );
}

const rowsWithActions = computed(() => {
  const users = (result.value?.users ?? []).filter(
    (user: any) => user && !isEmptyUser(user)
  );

  if (!filter.value) {
    return users.map((u: any) => ({ ...u, actions: "" }));
  }

  const searchTerm = filter.value.toLowerCase();
  const filtered = users.filter((user: any) =>
    Object.values(user).some((val) => val?.toString().toLowerCase().includes(searchTerm))
  );
  return filtered.map((u: any) => ({ ...u, actions: "" }));
});

/**
 * CRUD Methods
 */
async function createUser() {
  await createUserMutate({ input: newUser.value });
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
  await refetch();
}

async function deleteUser(id: string | number) {
  await deleteUserMutate({ id });
  await refetch();
}

function openModal(user: any) {
  if (!user) return;
  selectedUser.value = { ...user };
  isEditOpen.value = true;
}

async function updateUser() {
  if (!selectedUser.value) return;
  await updateUserMutate({
    input: {
      id: selectedUser.value.id,
      email: selectedUser.value.email,
      age: selectedUser.value.age,
      comment: selectedUser.value.comment,
      location: selectedUser.value.location,
      name: selectedUser.value.name,
      preferences: selectedUser.value.preferences,
    },
  });
  isEditOpen.value = false;
  await refetch();
}
</script>
