<?php

namespace Tests\Feature;

use App\Models\Comment;
use App\Models\CommentDislike;
use App\Models\CommentLike;
use App\Models\Dislike;
use App\Models\Favourite;
use App\Models\Follow;
use App\Models\Image;
use App\Models\Like;
use App\Models\Project;
use App\Models\Reply;
use App\Models\ReplyDislike;
use App\Models\ReplyLike;
use App\Models\Thread;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Support\Facades\DB;
use Tests\TestCase;

class DatabaseTest extends TestCase
{
    use RefreshDatabase;

    public function test_find_all_rows()
    {

        $this->expectsDatabaseQueryCount(70000);

        for ($i = 0; $i < 5000; $i++) {
            DB::table('users')->get();
            DB::table('projects')->get();
            DB::table('images')->get();
            DB::table('follows')->get();
            DB::table('favourites')->get();
            DB::table('comments')->get();
            DB::table('replies')->get();
            DB::table('threads')->get();
            DB::table('likes')->get();
            DB::table('dislikes')->get();
            DB::table('comment_likes')->get();
            DB::table('comment_dislikes')->get();
            DB::table('reply_likes')->get();
            DB::table('reply_dislikes')->get();
        }
    }

    public function test_user_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            User::all();
        }
    }
    public function test_project_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            Project::all();
        }
    }
    public function test_image_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            Image::all();
        }
    }
    public function test_follow_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            Follow::all();
        }
    }
    public function test_favourite_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            Favourite::all();
        }
    }
    public function test_comment_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            Comment::all();
        }
    }
    public function test_reply_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            Reply::all();
        }
    }
    public function test_thread_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            Thread::all();
        }
    }
    public function test_like_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            Like::all();
        }
    }
    public function test_dislike_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            Dislike::all();
        }
    }
    public function test_comment_like_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            CommentLike::all();
        }
    }
    public function test_comment_dislike_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            CommentDislike::all();
        }
    }
    public function test_reply_like_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            ReplyLike::all();
        }
    }
    public function test_reply_dislike_model()
    {
        $this->expectsDatabaseQueryCount(11000);

        for ($i = 0; $i < 11000; $i++) {
            ReplyDislike::all();
        }
    }
}
