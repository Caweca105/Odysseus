<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Models\User;
use Illuminate\Validation\Rule;

class UserController extends Controller
{
    /**
     * Display a listing of the resource.
     */
    public function index()
    {
        // Retrieve all users from the database
        $users = User::all();
        return response()->json($users);
    }

    /**
     * Show the form for creating a new resource.
     *
     * For a REST API you typically don't return an HTML form.
     * You can remove this method or return a 501 Not Implemented.
     */
    public function create()
    {
        return response()->json(['message' => 'Not implemented'], 501);
    }

    /**
     * Store a newly created resource in storage.
     */
    public function store(Request $request)
    {
        // Validate incoming request data
        $validatedData = $request->validate([
            'username'    => 'required|string|max:255',
            'email'       => 'required|string|email|max:255|unique:users',
            'age'         => 'nullable|integer',
            'comment'     => 'nullable|string',
            'location'    => 'nullable|string',
            'name'        => 'nullable|string|max:255',
            'preferences' => 'nullable|string',
        ]);

        // Create and persist the new user
        $user = User::create($validatedData);

        // Return the created user data with a 201 Created status
        return response()->json($user, 201);
    }

    /**
     * Display the specified resource.
     */
    public function show($id)
    {
        // Retrieve the user or return a 404 error if not found
        $user = User::findOrFail($id);
        return response()->json($user);
    }

    /**
     * Show the form for editing the specified resource.
     *
     * For a REST API you typically don't return an HTML form.
     * You can remove this method or return a 501 Not Implemented.
     */
    public function edit($id)
    {
        return response()->json(['message' => 'Not implemented'], 501);
    }

    /**
     * Update the specified resource in storage.
     */
    public function update(Request $request, $id)
    {
        // Find the user or return a 404 error if not found
        $user = User::findOrFail($id);

        // Validate the request data.
        // The 'sometimes' rule allows partial updates.
        $validatedData = $request->validate([
            'username'    => 'sometimes|required|string|max:255',
            'email'       => [
                'sometimes',
                'required',
                'string',
                'email',
                'max:255',
                Rule::unique('users')->ignore($user->id),
            ],
            'age'         => 'nullable|integer',
            'comment'     => 'nullable|string',
            'location'    => 'nullable|string',
            'name'        => 'nullable|string|max:255',
            'preferences' => 'nullable|string',
        ]);

        // Update the user with validated data
        $user->update($validatedData);

        return response()->json($user);
    }

    /**
     * Remove the specified resource from storage.
     */
    public function destroy($id)
    {
        // Find the user or return a 404 error if not found
        $user = User::findOrFail($id);
        $user->delete();

        // Return a 204 No Content response to indicate success
        return response()->json(null, 204);
    }
}
