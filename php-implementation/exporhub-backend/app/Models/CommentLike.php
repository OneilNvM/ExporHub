<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class CommentLike extends Model
{
    protected $table = 'comment_likes';
    protected $primaryKey = 'id';
    public $incrementing = true;

    public $timestamps = false;
}
