<?php

namespace Tests\Feature;

use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Support\Facades\DB;
use Tests\TestCase;

class DatabaseTest extends TestCase {
    use RefreshDatabase;

    public function test_find_all_rows() {

        $this->seed();
        
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

        print($users);

        $this->assertArrayHasKey("users", $results);
        
    }
}