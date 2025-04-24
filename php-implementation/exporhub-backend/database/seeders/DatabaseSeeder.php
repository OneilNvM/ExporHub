<?php

namespace Database\Seeders;

// use Illuminate\Database\Console\Seeds\WithoutModelEvents;
use Illuminate\Database\Seeder;
use Illuminate\support\Facades\DB;
use Illuminate\support\Facades\Hash;
use Faker\Factory as Faker;

class DatabaseSeeder extends Seeder
{
    /**
     * Seed the application's database.
     */
    public function run(): void
    {
        $faker = Faker::create();

        // Seed Users table
        $users = [];
        for ($i = 0; $i < 25; $i++) {
            $username = $faker->unique()->userName;
            $email = $faker->unique()->safeEmail;
            $users[] = [
                'username' => $username,
                'email' => $email,
                'password' => Hash::make('password'), // Default password for all users
                'bio' => $faker->paragraph(3),
                'profile_img' => $faker->imageUrl(200, 200, 'people'),
                'followers' => $faker->numberBetween(0, 500),
                'date_created' => now(),
            ];
        }
        DB::table('users')->insert($users);
        $user_ids = DB::table('users')->pluck('user_id')->toArray();

        // Seed Projects table
        $projects = [];
        for ($i = 0; $i < 25; $i++) {
            $projects[] = [
                'user_id' => $faker->randomElement($user_ids),
                'name' => $faker->unique()->sentence(3),
                'description' => $faker->paragraph(5),
                'favourites' => $faker->numberBetween(0, 200),
                'date_created' => now(),
                'date_updated' => $faker->optional()->dateTimeBetween('-1 month', 'now'),
            ];
        }
        DB::table('projects')->insert($projects);
        $project_ids = DB::table('projects')->pluck('project_id')->toArray();

        // Seed Images table
        $images = [];
        for ($i = 0; $i < 25; $i++) {
            $images[] = [
                'file_path' => $faker->unique()->imageUrl(),
                'user_id' => $faker->optional()->randomElement($user_ids),
                'project_id' => $faker->optional()->randomElement($project_ids),
                'date_uploaded' => now(),
            ];
        }
        DB::table('images')->insert($images);

        // Seed Follows table
        $follows = [];
        for ($i = 0; $i < 30; $i++) {
            $follower_id = $faker->randomElement($user_ids);
            $following_id = $faker->randomElement($user_ids);
            if ($follower_id !== $following_id && !in_array(['follower' => $follower_id, 'following' => $following_id], $follows)) {
                $follows[] = [
                    'follower' => $follower_id,
                    'following' => $following_id,
                    'date_followed' => now(),
                ];
            }
        }
        DB::table('follows')->insert($follows);

        // Seed Favourites table
        $favourites = [];
        for ($i = 0; $i < 30; $i++) {
            $favourites[] = [
                'user_id' => $faker->randomElement($user_ids),
                'project_id' => $faker->randomElement($project_ids),
                'date_favourited' => now(),
            ];
        }
        DB::table('favourites')->insert($favourites);

        // Seed Comments table
        $comments = [];
        for ($i = 0; $i < 25; $i++) {
            $comments[] = [
                'user_id' => $faker->randomElement($user_ids),
                'project_id' => $faker->randomElement($project_ids),
                'text' => $faker->sentence(10),
                'date' => now()->toDateString(),
                'replies' => $faker->numberBetween(0, 10),
            ];
        }
        DB::table('comments')->insert($comments);
        $comment_ids = DB::table('comments')->pluck('comment_id')->toArray();

        // Seed Replies table
        $replies = [];
        for ($i = 0; $i < 30; $i++) {
            $replies[] = [
                'user_id' => $faker->randomElement($user_ids),
                'text' => $faker->sentence(7),
                'date' => now()->toDateString(),
            ];
        }
        DB::table('replies')->insert($replies);
        $reply_ids = DB::table('replies')->pluck('reply_id')->toArray();

        // Seed Threads table
        $threads = [];
        for ($i = 0; $i < 30; $i++) {
            $threads[] = [
                'comment_id' => $faker->randomElement($comment_ids),
                'reply_id' => $faker->randomElement($reply_ids),
            ];
        }
        DB::table('threads')->insert($threads);

        // Seed Likes table
        $likes = [];
        for ($i = 0; $i < 30; $i++) {
            $likes[] = [
                'user_id' => $faker->randomElement($user_ids),
                'date' => now()->toDateString(),
            ];
        }
        DB::table('likes')->insert($likes);
        $like_ids = DB::table('likes')->pluck('like_id')->toArray();

        // Seed Dislikes table
        $dislikes = [];
        for ($i = 0; $i < 30; $i++) {
            $dislikes[] = [
                'user_id' => $faker->randomElement($user_ids),
                'date' => now()->toDateString(),
            ];
        }
        DB::table('dislikes')->insert($dislikes);
        $dislike_ids = DB::table('dislikes')->pluck('dislike_id')->toArray();

        // Seed Comment Likes table
        for ($i = 0; $i < 30; $i++) {
            DB::table('comment_likes')->insert([
                'comment_id' => $faker->randomElement($comment_ids),
                'like_id' => $faker->randomElement($like_ids),
            ]);
        }

        // Seed Comment Dislikes table
        for ($i = 0; $i < 30; $i++) {
            DB::table('comment_dislikes')->insert([
                'comment_id' => $faker->randomElement($comment_ids),
                'dislike_id' => $faker->randomElement($dislike_ids),
            ]);
        }

        // Seed Reply Likes table
        for ($i = 0; $i < 30; $i++) {
            DB::table('reply_likes')->insert([
                'reply_id' => $faker->randomElement($reply_ids),
                'like_id' => $faker->randomElement($like_ids),
            ]);
        }

        // Seed Reply Dislikes table
        for ($i = 0; $i < 30; $i++) {
            DB::table('reply_dislikes')->insert([
                'reply_id' => $faker->randomElement($reply_ids),
                'dislike_id' => $faker->randomElement($dislike_ids),
            ]);
        }
    }
}
