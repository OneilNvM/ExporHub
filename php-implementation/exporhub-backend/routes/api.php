<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Hash;

Route::get(
    '/',
    fn(Request $request) => $request->user
);

Route::get('/all', function () {
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

Route::get('/users', function () {
    $users = DB::table('users')->get();

    return $users;
});

Route::get('/projects', function () {
    $projects = DB::table('projects')->get();

    return $projects;
});

Route::get('/images', function () {
    $images = DB::table('images')->get();

    return $images;
});

Route::get('/follows', function () {
    $follows = DB::table('follows')->get();

    return $follows;
});

Route::get('/favourites', function () {
    $favourites = DB::table('favourites')->get();

    return $favourites;
});

Route::get('/comments', function () {
    $comments = DB::table('comments')->get();

    return $comments;
});

Route::get('/replies', function () {
    $replies = DB::table('replies')->get();

    return $replies;
});

Route::get('/threads', function () {
    $threads = DB::table('threads')->get();

    return $threads;
});

Route::get('/likes', function () {
    $likes = DB::table('likes')->get();

    return $likes;
});

Route::get('/dislikes', function () {
    $dislikes = DB::table('dislikes')->get();

    return $dislikes;
});

Route::get('/comment-likes', function () {
    $comment_likes = DB::table('comment_likes')->get();

    return $comment_likes;
});

Route::get('/comment-dislikes', function () {
    $comment_dislikes = DB::table('comment_dislikes')->get();

    return $comment_dislikes;
});

Route::get('/reply-likes', function () {
    $reply_likes = DB::table('reply_likes')->get();

    return $reply_likes;
});

Route::get('/reply-dislikes', function () {
    $reply_dislikes = DB::table('reply_dislikes')->get();

    return $reply_dislikes;
});

Route::match(['post', 'options'], '/signin', function (Request $request) {
    $identity = $request->input('username_or_email');

    if (str_contains($identity, "@")) {
        $user = DB::table('users')->where('email', $identity)->firstOrFail();

        $password = Hash::check($request->input('password'), $user->password);

        return $password ? $user : response('Invalid credentials');
    } else {
        $user = DB::table('users')->where('username', $identity)->firstOrFail();

        $password = Hash::check($request->input('password'), $user->password);

        return $password ? $user : response('Invalid credentials');
    }
});

Route::get('/user/user-id', function (Request $request) {
    $user_id = $request->query("user_id");

    $user = DB::table('users')->where('user_id', intval($user_id))->firstOrFail();

    return $user;
});

Route::get('/user/username', function (Request $request) {
    $username = $request->query('username');

    $user = DB::table('users')->where('username', $username)->firstOrFail();

    return $user;
});

Route::get('/user/email', function (Request $request) {
    $email = $request->query('email');

    $user = DB::table('users')->where('email', $email)->firstOrFail();

    return $user;
});

Route::get('/project/project-id', function (Request $request) {
    $project_id = $request->query('project_id');

    $project = DB::table('projects')->where('project_id', intval($project_id))->firstOrFail();

    return $project;
});

Route::get('/project/user-id', function (Request $request) {
    $user_id = $request->query('user_id');

    $projects = DB::table('projects')->where('user_id', intval($user_id))->get();

    return $projects;
});

Route::get('/project/date-updated', function (Request $request) {
    $user_id = $request->query('user_id');

    $projects = DB::table('projects')->where('user_id', intval($user_id))->whereNotNull('date_updated')->orderBy('date_updated')->get();

    return $projects;
});

Route::get('/project/num-of-projects', function (Request $request) {
    $user_id = $request->query('user_id');

    $count = DB::table('projects')->where('user_id', intval($user_id))->count();

    return $count;
});

Route::get('/favourite/user-id', function (Request $request) {
    $user_id = $request->query('user_id');

    $favourites = DB::table('favourites')->where('user_id', intval($user_id))->get();

    return $favourites;
});

Route::get('/favourite/u-p-id', function (Request $request) {
    $user_id = $request->query('user_id');
    $project_id = $request->query('project_id');

    $favourite = DB::table('favourites')->where('user_id', intval($user_id))->where('project_id', intval($project_id))->firstOrFail();

    return $favourite;
});

Route::get('/follow/user-id', function (Request $request) {
    $user_id = $request->query('user_id');

    $follows = DB::table('follows')->where('follower', intval($user_id))->get();

    return $follows;
});

Route::get('/follow/unique-follow', function (Request $request) {
    $follower = $request->query('follower');
    $following = $request->query('following');

    $follow = DB::table('follows')->where('follower', intval($follower))->where('following', intval($following))->firstOrFail();

    return $follow;
});

Route::get('/image/profile-image', function (Request $request) {
    $user_id = $request->query('user_id');

    $image = DB::table('images')->where('user_id', intval($user_id))->firstOrFail();

    return $image;
});

Route::get('/image/project-images', function (Request $request) {
    $user_id = $request->query('user_id');
    $project_id = $request->query('project_id');

    $images = DB::table('images')->where('user_id', intval($user_id))->where('project_id', intval($project_id))->get();

    return $images;
});

Route::get('/like/user-likes', function (Request $request) {
    $user_id = $request->query('user_id');

    $likes = DB::table('likes')->where('user_id', intval($user_id))->get();

    return $likes;
});

Route::get('/like/comment-likes', function (Request $request) {
    $comment_id = $request->query('comment_id');

    $likes = DB::table('comment_likes')->where('comment_id', intval($comment_id))->get();

    return $likes;
});

Route::get('/like/reply-likes', function (Request $request) {
    $reply_id = $request->query('reply_id');

    $likes = DB::table('reply_likes')->where('reply_id', intval($reply_id))->get();

    return $likes;
});

Route::get('/dislike/user-dislikes', function (Request $request) {
    $user_id = $request->query('user_id');

    $dislikes = DB::table('dislikes')->where('user_id', intval($user_id))->get();

    return $dislikes;
});

Route::get('/dislike/comment-dislikes', function (Request $request) {
    $comment_id = $request->query('comment_id');

    $dislikes = DB::table('comment_dislikes')->where('comment_id', intval($comment_id))->get();

    return $dislikes;
});

Route::get('/dislike/reply-dislikes', function (Request $request) {
    $reply_id = $request->query('reply_id');

    $dislikes = DB::table('reply_dislikes')->where('reply_id', intval($reply_id))->get();

    return $dislikes;
});

Route::get('/search/query', function (Request $request) {
    $query = $request->query('q');

    $users = DB::table('users')->whereFullText(['username', 'bio'], $query)->get();
    $projects = DB::table('projects')->whereFullText(['name', 'description'], $query)->get();

    $results = [
        'Users' => $users,
        'Projects' => $projects
    ];

    return $results;
});

Route::get('/comment/project-comments', function (Request $request) {
    $project_id = $request->query('project_id');

    $comments = DB::table('comments')->where('project_id', intval($project_id))->get();

    return $comments;
});

Route::get('/reply/thread-replies', function (Request $request) {
    $comment_id = $request->query('comment_id');

    $threads = DB::table('threads')->where('comment_id', intval($comment_id))->get();

    $replies = [];

    for ($i = 0; $i < $threads->count(); $i++) {
        $reply = DB::table('replies')->where('reply_id', $threads->get($i)->{'reply_id'})->firstOrFail();

        array_push($replies, $reply);
    }

    return $replies;
});