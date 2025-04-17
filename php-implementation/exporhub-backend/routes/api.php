<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Hash;

Route::middleware(['auth:sanctum'])->get('/user', function (Request $request) {
    return $request->user();
});

Route::get('/', fn(Request $request) => $request->user
)->middleware('auth:sanctum');

Route::get('/all', function() {
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

    return $results;
});

Route::get('/users', function() {
    $users = DB::table('users')->get();

    return $users;
});

Route::get('/projects', function() {
    $projects = DB::table('projects')->get();

    return $projects;
});

Route::get('/images', function() {
    $images = DB::table('images')->get();

    return $images;
});

Route::get('/follows', function() {
    $follows = DB::table('follows')->get();

    return $follows;
});

Route::get('/favourites', function() {
    $favourites = DB::table('favourites')->get();

    return $favourites;
});

Route::get('/comments', function() {
    $comments = DB::table('comments')->get();

    return $comments;
});

Route::get('/replies', function() {
    $replies = DB::table('replies')->get();

    return $replies;
});

Route::get('/threads', function() {
    $threads = DB::table('threads')->get();

    return $threads;
});

Route::get('/likes', function() {
    $likes = DB::table('likes')->get();

    return $likes;
});

Route::get('/dislikes', function() {
    $dislikes = DB::table('dislikes')->get();

    return $dislikes;
});

Route::get('/comment-likes', function() {
    $comment_likes = DB::table('comment_likes')->get();

    return $comment_likes;
});

Route::get('/comment-dislikes', function() {
    $comment_dislikes = DB::table('comment_dislikes')->get();

    return $comment_dislikes;
});

Route::get('/reply-likes', function() {
    $reply_likes = DB::table('reply_likes')->get();

    return $reply_likes;
});

Route::get('/reply-dislikes', function() {
    $reply_dislikes = DB::table('reply_dislikes')->get();

    return $reply_dislikes;
});

Route::post('/login', function(Request $request) {
    $identity = $request->input('username_or_email');

    // if (str_contains($identity, "@")) {
    //     $user = DB::table('users')->where('username', $identity)->firstOrFail();

    //     if ($user) {
    //         return 'Invalid credentials';
    //     } else {
    //         $password = Hash::check($request->input('password'), $user->password);
            
    //         return $password ? $user : 'Invalid credentials';    
    //     }
    // } else {
    //     $user = DB::table('users')->where('email', $identity)->firstOrFail();

    //     if ($user->isEmpty()) {
    //         return 'Invalid credentials';
    //     } else {
    //         $password = Hash::check($request->input('password'), $user->password);

    //         return $password ? $user : 'Invalid credentials';  
    //     }
    // }

    return $identity;
});

