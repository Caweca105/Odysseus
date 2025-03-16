<template>
  <UContainer class="p-4">
    <h1>Django with GraphQL</h1>
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
        <UButton
          label="Create"
          @click="isCreateOpen = true"
          :ui="{ rounded: 'rounded-full' }"
          class="ml-4"
        />
      </div>

      <!-- Table with Filtered Rows -->
      <UTable :rows="rowsWithActions" :columns="columns">
        <template #actions-data="{ row }">
          <div class="flex gap-1 justify-end">
            <UButton
              @click="openModal(row)"
              :ui="{ rounded: 'rounded-full' }"
              icon="i-heroicons-pencil-square"
            />
            <UButton
              @click="deleteUser(Number(row.id))"
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

const isCreateOpen = ref(false);
const isEditOpen = ref(false);
const selectedUser = ref<any>(null);

// GraphQL Query to fetch all users from the Django API
const GET_USERS = gql`
  query GetUsers {
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

// GraphQL Mutations corresponding to our Django API schema
const CREATE_USER = gql`
  mutation CreateUser($input: NewUserInput!) {
    createUser(input: $input) {
      user {
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
  }
`;

const UPDATE_USER = gql`
  mutation UpdateUser($input: UpdateUserInput!) {
    updateUser(input: $input) {
      user {
        id
        username
        email
        age
        comment
        location
        name
        preferences
      }
      ok
    }
  }
`;

const DELETE_USER = gql`
  mutation DeleteUser($id: Int!) {
    deleteUser(id: $id) {
      ok
    }
  }
`;

const { result, loading, error, refetch } = useQuery(GET_USERS, null, {
  clientId: "DjangoGraphQL",
});
// Similarly, if you need to use the Django endpoint for mutations, add the clientId option:
const { mutate: createUserMutate } = useMutation(CREATE_USER, {
  clientId: "DjangoGraphQL",
});

const { mutate: updateUserMutate } = useMutation(UPDATE_USER, {
  clientId: "DjangoGraphQL",
});

const { mutate: deleteUserMutate } = useMutation(DELETE_USER, {
  clientId: "DjangoGraphQL",
});

const newUser = ref({
  username: "",
  email: "",
  age: null as number | null,
  comment: "",
  location: "",
  name: "",
  preferences: "",
});

const columns = [
  { key: "username", label: "Username" },
  { key: "email", label: "Email" },
  { key: "age", label: "Age" },
  { key: "location", label: "Location" },
  { key: "name", label: "Name" },
  { key: "actions", label: "Actions" },
];

const isEmptyUser = (user: any) => {
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
};

// Store the filter input
const filter = ref("");

// Compute filtered rows and add an "actions" property so that the table renders the actions slot.
const rowsWithActions = computed(() => {
  const users = (result.value?.users ?? []).filter(
    (user: any) => user && !isEmptyUser(user)
  );
  const filtered = !filter.value
    ? users
    : users.filter((user: any) => {
        const searchTerm = filter.value.toLowerCase();
        return Object.values(user).some((value) =>
          value?.toString().toLowerCase().includes(searchTerm)
        );
      });
  return filtered.map((user: any) => ({ ...user, actions: "" }));
});

const createUser = async () => {
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
};

const deleteUser = async (id: number) => {
  await deleteUserMutate({ id });
  await refetch();
};

const openModal = (user: any) => {
  if (!user) return;
  selectedUser.value = { ...user };
  isEditOpen.value = true;
};

const updateUser = async () => {
  if (!selectedUser.value) return;

  await updateUserMutate({
    input: {
      id: Number(selectedUser.value.id),
      email: selectedUser.value.email,
      age: selectedUser.value.age,
      comment: selectedUser.value.comment,
      location: selectedUser.value.location,
      name: selectedUser.value.name,
      preferences: selectedUser.value.preferences,
    },
  });
  console.log("Update response:", response);
  isEditOpen.value = false;
  await refetch();
};
</script>
