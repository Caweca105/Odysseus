<template>
  <div>
    <h1>User CRUD</h1>
    <!-- List Users -->
    <div v-if="loading">Loading users…</div>
    <div v-else-if="error">Error: {{ error.message }}</div>
    <div v-else>
      <h2>Users</h2>
      <ul>
        <li v-for="user in result?.users" :key="user.id">
          <strong>{{ user.username }}</strong> ({{ user.email }})
          <button @click="deleteUser(user.id)">Delete</button>
          <button @click="editUser(user)">Edit</button>
        </li>
      </ul>
    </div>

    <!-- Create User Form -->
    <h2>Create New User</h2>
    <form @submit.prevent="createUser">
      <div>
        <label>Username:</label>
        <input v-model="newUser.username" type="text" required />
      </div>
      <div>
        <label>Email:</label>
        <input v-model="newUser.email" type="email" required />
      </div>
      <div>
        <label>Age:</label>
        <input v-model.number="newUser.age" type="number" />
      </div>
      <div>
        <label>Comment:</label>
        <input v-model="newUser.comment" type="text" />
      </div>
      <div>
        <label>Location:</label>
        <input v-model="newUser.location" type="text" />
      </div>
      <div>
        <label>Name:</label>
        <input v-model="newUser.name" type="text" />
      </div>
      <div>
        <label>Preferences:</label>
        <input v-model="newUser.preferences" type="text" />
      </div>
      <button type="submit">Create User</button>
    </form>

    <!-- Edit User Form -->
    <div v-if="editingUser">
      <h2>Edit User</h2>
      <form @submit.prevent="updateUser">
        <div>
          <label>Email:</label>
          <input v-model="editingUser.email" type="email" required />
        </div>
        <div>
          <label>Age:</label>
          <input v-model.number="editingUser.age" type="number" />
        </div>
        <div>
          <label>Comment:</label>
          <input v-model="editingUser.comment" type="text" />
        </div>
        <div>
          <label>Location:</label>
          <input v-model="editingUser.location" type="text" />
        </div>
        <div>
          <label>Name:</label>
          <input v-model="editingUser.name" type="text" />
        </div>
        <div>
          <label>Preferences:</label>
          <input v-model="editingUser.preferences" type="text" />
        </div>
        <button type="submit">Update User</button>
        <button type="button" @click="editingUser = null">Cancel</button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useQuery, useMutation } from "@vue/apollo-composable";
import gql from "graphql-tag";

// Query to fetch all users
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

// Mutation to create a user
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

// Mutation to update a user
const UPDATE_USER = gql`
  mutation UpdateUser($input: UpdateUser!) {
    update_user(input: $input) {
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

// Mutation to delete a user
const DELETE_USER = gql`
  mutation DeleteUser($id: Int!) {
    delete_user(id: $id)
  }
`;

// Reactive variables for form data
const newUser = ref({
  username: "",
  email: "",
  age: null as number | null,
  comment: "",
  location: "",
  name: "",
  preferences: "",
});
const editingUser = ref(null as any);

// Query to fetch users
const { result, loading, error, refetch } = useQuery(GET_USERS);

// Mutation functions
const { mutate: createUserMutate } = useMutation(CREATE_USER);
const { mutate: updateUserMutate } = useMutation(UPDATE_USER);
const { mutate: deleteUserMutate } = useMutation(DELETE_USER);

// Create a new user
const createUser = async () => {
  await createUserMutate({
    input: {
      username: newUser.value.username,
      email: newUser.value.email,
      age: newUser.value.age,
      comment: newUser.value.comment,
      location: newUser.value.location,
      name: newUser.value.name,
      preferences: newUser.value.preferences,
    },
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
  await refetch();
};

// Delete a user by id
const deleteUser = async (id: number) => {
  await deleteUserMutate({ id });
  await refetch();
};

// Select a user for editing
const editUser = (user: any) => {
  editingUser.value = { ...user };
};

// Update an existing user
const updateUser = async () => {
  await updateUserMutate({
    input: {
      id: editingUser.value.id,
      email: editingUser.value.email,
      age: editingUser.value.age,
      comment: editingUser.value.comment,
      location: editingUser.value.location,
      name: editingUser.value.name,
      preferences: editingUser.value.preferences,
    },
  });
  editingUser.value = null;
  await refetch();
};

// Optionally log your GraphQL endpoint if using Nuxt's runtime config
console.log("GraphQL Endpoint:", useRuntimeConfig().public.graphqlEndpoint);
</script>

<style scoped>
div {
  margin: 10px;
}
label {
  display: inline-block;
  width: 100px;
}
input {
  margin-bottom: 5px;
}
button {
  margin-left: 5px;
}
</style>
