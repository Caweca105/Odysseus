<template>
  <UContainer class="p-4">
    <h1>Rust With GraphQL</h1>
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
            <UModal v-model:open="editModalOpen">
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
import { useQuery, useMutation } from "@vue/apollo-composable";
import gql from "graphql-tag";
import type { TableColumn, DropdownMenuItem } from "@nuxt/ui";

const editModalOpen = ref(false);
const selectedUser = ref<any>(null);

const defaultNewUser = {
  username: "",
  email: "",
  age: null as number | null,
  comment: "",
  location: "",
  name: "",
  preferences: "",
};
const newUser = ref({ ...defaultNewUser });

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

const CREATE_USER = gql`
  mutation CreateUser($input: NewUser!) {
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
  mutation UpdateUser($input: UpdateUser!) {
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

const { result, loading, error, refetch } = useQuery(GET_USERS, null, {
  clientId: "RustGraphQL",
});
const { mutate: createUserMutate } = useMutation(CREATE_USER, {
  clientId: "RustGraphQL",
});
const { mutate: updateUserMutate } = useMutation(UPDATE_USER, {
  clientId: "RustGraphQL",
});
const { mutate: deleteUserMutate } = useMutation(DELETE_USER, {
  clientId: "RustGraphQL",
});

type User = {
  id: string;
  name: string;
  username: string;
  email: string;
  age: number;
  location: string;
  preferences: string;
};

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

function getDropdownActions(user: User): DropdownMenuItem[][] {
  return [
    [
      {
        label: "Edit",
        icon: "i-lucide-edit",
        onSelect: () => {
          selectedUser.value = { ...user };
          editModalOpen.value = true;
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

const isEmptyUser = (user: any) =>
  !user.username?.trim() &&
  !user.email?.trim() &&
  user.age === null &&
  !user.comment?.trim() &&
  !user.location?.trim() &&
  !user.name?.trim() &&
  !user.preferences?.trim();

const filter = ref("");

const rowsWithActions = computed(() => {
  const users = (result.value?.users ?? [])
    .filter((user: any) => user && !isEmptyUser(user))
    .sort((a: User, b: User) => Number(a.id) - Number(b.id));

  const filtered = !filter.value
    ? users
    : users.filter((user: any) =>
        Object.values(user).join(" ").toLowerCase().includes(filter.value.toLowerCase())
      );

  return filtered.map((user: any) => ({ ...user, actions: "" }));
});

const createUser = async () => {
  await createUserMutate({ input: newUser.value });
  newUser.value = { ...defaultNewUser };
  await refetch();
};

const deleteUser = async (id: string) => {
  await deleteUserMutate({ id });
  await refetch();
};

const updateUser = async () => {
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
  await refetch();
  editModalOpen.value = false;
};
</script>
