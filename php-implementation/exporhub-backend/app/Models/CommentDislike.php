<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class CommentDislike extends Model
{
    protected $table = 'comment_dislikes';
    protected $primaryKey = 'id';
    public $incrementing = true;

    public $timestamps = false;
}
