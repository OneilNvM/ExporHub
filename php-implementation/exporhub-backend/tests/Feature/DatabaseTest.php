<?php

namespace Tests\Feature;

use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Support\Facades\DB;
use Tests\TestCase;

class DatabaseTest extends TestCase
{
    use RefreshDatabase;

    public function test_find_all_rows()
    {

        $this->expectsDatabaseQueryCount(14);

        $users = DB::table('users')->get();
        $projects = DB::table('projects')->get();
        $images = DB::table('images')->get();
        $follows = DB::table('follows')->get();
        $favourites = DB::table('favourites')->get();
        $comments = DB::table('comments')->get();
        $replies = DB::table('replies')->get();
        $threads = DB::table('threads')->get();
        $likes = DB::table('likes')->get();
        $dislikes = DB::table('dislikes')->get();
        $comment_likes = DB::table('comment_likes')->get();
        $comment_dislikes = DB::table('comment_dislikes')->get();
        $reply_likes = DB::table('reply_likes')->get();
        $reply_dislikes = DB::table('reply_dislikes')->get();

        $results = [
            "users" => $users,
            "projects" => $projects,
            "images" => $images,
            "follows" => $follows,
            "favourites" => $favourites,
            "comments" => $comments,
            "replies" => $replies,
            "threads" => $threads,
            "likes" => $likes,
            "dislikes" => $dislikes,
            "comment_likes" => $comment_likes,
            "comment_dislikes" => $comment_dislikes,
            "reply_likes" => $reply_likes,
            "reply_dislikes" => $reply_dislikes,
        ];

        clock($results);

        assert(count($results) == 14);
    }

    public function test_user_model()
    {
        $this->expectsDatabaseQueryCount(7501);

        $user1 = User::where('user_id', 1)->firstOrFail();

        clock($user1);

        for ($i = 0; $i < 500; $i++) {
            $user2 = User::where('username', $user1->username)->firstOrFail();

            print$user2;
        }

        for ($i = 0; $i < 1000; $i++) {
            $user2 = User::where('email', $user1->email)->firstOrFail();

            print$user2;
        }

        for ($i = 0; $i < 1500; $i++) {
            $user2 = User::where('username', $user1->username)->firstOrFail();

            print$user2;
        }

        for ($i = 0; $i < 2000; $i++) {
            $user2 = User::where('email', $user1->email)->firstOrFail();

            print$user2;
        }

        for ($i = 0; $i < 2500; $i++) {
            $user2 = User::where('email', $user1->email)->firstOrFail();

            print$user2;
        }
    }
}
