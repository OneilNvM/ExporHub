<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class ReplyDislike extends Model
{
    protected $table = 'reply_dislikes';
    protected $primaryKey = 'id';
    public $incrementing = true;

    public $timestamps = false;
}
