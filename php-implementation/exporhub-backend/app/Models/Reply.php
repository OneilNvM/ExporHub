<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class Reply extends Model
{
    protected $table = 'replies';
    protected $primaryKey = 'reply_id';
    public $incrementing = true;

    public $timestamps = false;
}
