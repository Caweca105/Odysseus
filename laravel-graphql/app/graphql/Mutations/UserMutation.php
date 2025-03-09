<?php

namespace App\GraphQL\Mutations;

use App\Models\User;

class UserMutation
{
    public function createUser($_, array $args)
    {
        // "input" is the argument name from the schema
        $input = $args['input'];

        // Create a new user
        return User::create([
            'username'   => $input['username'],
            'email'      => $input['email'],
            'age'        => $input['age']        ?? null,
            'comment'    => $input['comment']    ?? null,
            'location'   => $input['location']   ?? null,
            'name'       => $input['name']       ?? null,
            'preferences'=> $input['preferences']?? null,
        ]);
    }

    public function updateUser($_, array $args)
    {
        $input = $args['input'];

        // Find the existing user or fail
        $user = User::findOrFail($input['id']);

        // Update only the fields provided
        if (isset($input['username'])) {
            $user->username = $input['username'];
        }
        if (isset($input['email'])) {
            $user->email = $input['email'];
        }
        if (array_key_exists('age', $input)) {
            $user->age = $input['age'];
        }
        if (array_key_exists('comment', $input)) {
            $user->comment = $input['comment'];
        }
        if (array_key_exists('location', $input)) {
            $user->location = $input['location'];
        }
        if (array_key_exists('name', $input)) {
            $user->name = $input['name'];
        }
        if (array_key_exists('preferences', $input)) {
            $user->preferences = $input['preferences'];
        }

        $user->save();
        return $user;
    }

    public function deleteUser($_, array $args)
    {
        $user = User::find($args['id']);
        if (!$user) {
            return false;
        }

        $user->delete();
        return true;
    }
}
