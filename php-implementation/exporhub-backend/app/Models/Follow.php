<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class Follow extends Model
{
    protected $table = 'follows';
    protected $primaryKey = 'follow_id';
    public $incrementing = true;

    public $timestamps = false;
}
