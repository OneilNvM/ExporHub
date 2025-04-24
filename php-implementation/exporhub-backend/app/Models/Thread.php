<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class Thread extends Model
{
    protected $table = 'threads';
    protected $primaryKey = 'thread_id';
    public $incrementing = true;

    public $timestamps = false;
}
