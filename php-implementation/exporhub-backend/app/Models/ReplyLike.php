<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class ReplyLike extends Model
{
    protected $table = 'reply_likes';
    protected $primaryKey = 'id';
    public $incrementing = true;

    public $timestamps = false;
}
