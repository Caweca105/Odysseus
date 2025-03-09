<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class User extends Model
{
    protected $table = 'users';

    // Allow mass-assignment for these columns
    protected $fillable = [
        'username',
        'email',
        'age',
        'comment',
        'location',
        'name',
        'preferences',
    ];
}
