<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class Dislike extends Model
{
    protected $table = 'dislikes';
    protected $primaryKey = 'dislike_id';
    public $incrementing = true;

    public $timestamps = false;
}
