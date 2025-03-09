<?php

namespace App\GraphQL\Queries;

use App\Models\User;

class UserQuery
{
    public function user($_, array $args)
    {
        // Find a single user by ID
        return User::find($args['id']);
    }

    public function users($_, array $args)
    {
        // Return all users
        return User::all();
    }
}
