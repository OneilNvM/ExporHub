<?php

namespace Database\Seeders;

// use Illuminate\Database\Console\Seeds\WithoutModelEvents;
use Illuminate\Database\Seeder;
use Illuminate\support\Facades\DB;
use Illuminate\support\Facades\Hash;
use Carbon\Carbon;

class DatabaseSeeder extends Seeder
{
    /**
     * Seed the application's database.
     */
    public function run(): void
    {
        // Users
        $users = [
            [
                'username' => 'john_doe',
                'email' => 'john.doe@example.com',
                'password' => Hash::make('password123'),
                'bio' => 'A passionate developer and creator.',
                'profile_img' => 'uploads/profiles/john.jpg',
                'followers' => 150,
                'date_created' => Carbon::now(),
            ],
            [
                'username' => 'jane_smith',
                'email' => 'jane.smith@example.com',
                'password' => Hash::make('secure_pass'),
                'bio' => 'Loves designing beautiful and functional things.',
                'profile_img' => 'uploads/profiles/jane.png',
                'followers' => 210,
                'date_created' => Carbon::now()->subDays(5),
            ],
            [
                'username' => 'peter_pan',
                'email' => 'peter.pan@neverland.com',
                'password' => Hash::make('flyhigh'),
                'bio' => 'Always young at heart.',
                'profile_img' => null,
                'followers' => 50,
                'date_created' => Carbon::now()->subWeeks(2),
            ],
            [
                'username' => 'alice_wonder',
                'email' => 'alice@wonderland.net',
                'password' => Hash::make('downtherabbithole'),
                'bio' => 'Curious and always exploring.',
                'profile_img' => 'uploads/profiles/alice.gif',
                'followers' => 300,
                'date_created' => Carbon::now()->subMonths(1),
            ],
        ];
        DB::table('users')->insert($users);

        // Projects
        $projects = [
            [
                'name' => 'Awesome Portfolio Website',
                'description' => 'My personal portfolio showcasing my skills and projects.',
                'favourites' => 25,
                'user_id' => 1,
                'date_created' => Carbon::now()->subDays(2),
                'date_updated' => Carbon::now(),
            ],
            [
                'name' => 'Mobile App Concept',
                'description' => 'A concept for a new social networking mobile application.',
                'favourites' => 55,
                'user_id' => 2,
                'date_created' => Carbon::now()->subDays(7),
                'date_updated' => Carbon::now()->subDays(3),
            ],
            [
                'name' => 'Open Source Library',
                'description' => 'A collection of useful functions for PHP development.',
                'favourites' => 120,
                'user_id' => 1,
                'date_created' => Carbon::now()->subWeeks(3),
                'date_updated' => null,
            ],
            [
                'name' => 'Creative Illustration Series',
                'description' => 'A series of digital illustrations exploring different themes.',
                'favourites' => 80,
                'user_id' => 2,
                'date_created' => Carbon::now()->subMonths(1)->addDays(5),
                'date_updated' => Carbon::now()->subDays(10),
            ],
            [
                'name' => 'Data Analysis Dashboard',
                'description' => 'A dashboard for visualizing key business metrics.',
                'favourites' => 30,
                'user_id' => 3,
                'date_created' => Carbon::now()->subDays(10),
                'date_updated' => Carbon::now()->subDays(1),
            ],
        ];
        DB::table('projects')->insert($projects);

        // Images
        $images = [
            [
                'file_path' => 'uploads/images/portfolio_screenshot.png',
                'user_id' => 1,
                'project_id' => 1,
                'date_uploaded' => Carbon::now()->subDays(2),
            ],
            [
                'file_path' => 'uploads/images/mobile_app_ui.jpg',
                'user_id' => 2,
                'project_id' => 2,
                'date_uploaded' => Carbon::now()->subDays(6),
            ],
            [
                'file_path' => 'uploads/images/library_logo.svg',
                'user_id' => 1,
                'project_id' => 3,
                'date_uploaded' => Carbon::now()->subWeeks(3),
            ],
            [
                'file_path' => 'uploads/images/illustration_01.jpg',
                'user_id' => 2,
                'project_id' => 4,
                'date_uploaded' => Carbon::now()->subMonths(1)->addDays(6),
            ],
            [
                'file_path' => 'uploads/images/dashboard_view.png',
                'user_id' => 3,
                'project_id' => 5,
                'date_uploaded' => Carbon::now()->subDays(9),
            ],
            [
                'file_path' => 'uploads/profiles/john_full.jpg',
                'user_id' => 1,
                'project_id' => null,
                'date_uploaded' => Carbon::now()->subDays(3),
            ],
            [
                'file_path' => 'uploads/profiles/jane_full.png',
                'user_id' => 2,
                'project_id' => null,
                'date_uploaded' => Carbon::now()->subDays(8),
            ],
        ];
        DB::table('images')->insert($images);

        // Follows
        $follows = [
            ['follower' => 1, 'following' => 2, 'date_followed' => Carbon::now()->subDays(1)],
            ['follower' => 1, 'following' => 4, 'date_followed' => Carbon::now()->subDays(4)],
            ['follower' => 2, 'following' => 1, 'date_followed' => Carbon::now()->subDays(3)],
            ['follower' => 3, 'following' => 1, 'date_followed' => Carbon::now()->subWeeks(1)],
            ['follower' => 4, 'following' => 2, 'date_followed' => Carbon::now()->subDays(7)],
        ];
        DB::table('follows')->insert($follows);

        // Favourites
        $favourites = [
            ['user_id' => 2, 'project_id' => 1, 'date_favourited' => Carbon::now()->subDays(1)],
            ['user_id' => 1, 'project_id' => 2, 'date_favourited' => Carbon::now()->subDays(4)],
            ['user_id' => 4, 'project_id' => 1, 'date_favourited' => Carbon::now()->subDays(2)],
            ['user_id' => 2, 'project_id' => 3, 'date_favourited' => Carbon::now()->subWeeks(1)],
            ['user_id' => 3, 'project_id' => 4, 'date_favourited' => Carbon::now()->subDays(5)],
        ];
        DB::table('favourites')->insert($favourites);

        // Comments
        $comments = [
            [
                'text' => 'This is a fantastic portfolio!',
                'user_id' => 2,
                'project_id' => 1,
                'date' => Carbon::now()->subDays(1)->toDateString(),
                'replies' => 1,
            ],
            [
                'text' => 'Great concept for the mobile app.',
                'user_id' => 1,
                'project_id' => 2,
                'date' => Carbon::now()->subDays(3)->toDateString(),
                'replies' => 0,
            ],
            [
                'text' => 'Love the clean design of the library logo.',
                'user_id' => 4,
                'project_id' => 3,
                'date' => Carbon::now()->subWeeks(1)->toDateString(),
                'replies' => 0,
            ],
            [
                'text' => 'The illustration series is very inspiring.',
                'user_id' => 3,
                'project_id' => 4,
                'date' => Carbon::now()->subDays(6)->toDateString(),
                'replies' => 2,
            ],
            [
                'text' => 'Useful dashboard!',
                'user_id' => 1,
                'project_id' => 5,
                'date' => Carbon::now()->subDays(2)->toDateString(),
                'replies' => 0,
            ],
        ];
        DB::table('comments')->insert($comments);

        // Replies
        $replies = [
            [
                'text' => 'Thank you!',
                'user_id' => 1,
                'date' => Carbon::now()->subDays(1)->toDateString(),
            ],
            [
                'text' => 'Glad you like it!',
                'user_id' => 2,
                'date' => Carbon::now()->subDays(5)->toDateString(),
            ],
            [
                'text' => 'Which one is your favorite?',
                'user_id' => 2,
                'date' => Carbon::now()->subDays(4)->toDateString(),
            ],
        ];
        DB::table('replies')->insert($replies);

        // Threads (linking comments and replies)
        $threads = [
            ['comment_id' => 1, 'reply_id' => 1],
            ['comment_id' => 4, 'reply_id' => 2],
            ['comment_id' => 4, 'reply_id' => 3],
        ];
        DB::table('threads')->insert($threads);

        // Likes
        $likes = [
            ['user_id' => 3, 'date' => Carbon::now()->subDays(1)->toDateString()],
            ['user_id' => 4, 'date' => Carbon::now()->subDays(3)->toDateString()],
            ['user_id' => 1, 'date' => Carbon::now()->subWeeks(1)->toDateString()],
            ['user_id' => 2, 'date' => Carbon::now()->subDays(5)->toDateString()],
        ];
        DB::table('likes')->insert($likes);

        // Dislikes
        $dislikes = [
            ['user_id' => 2, 'date' => Carbon::now()->subDays(2)->toDateString()],
            ['user_id' => 3, 'date' => Carbon::now()->subDays(4)->toDateString()],
        ];
        DB::table('dislikes')->insert($dislikes);

        // Comment Likes
        $commentLikes = [
            ['comment_id' => 1, 'like_id' => 1],
            ['comment_id' => 1, 'like_id' => 2],
            ['comment_id' => 2, 'like_id' => 3],
        ];
        DB::table('comment_likes')->insert($commentLikes);

        // Comment Dislikes
        $commentDislikes = [
            ['comment_id' => 3, 'dislike_id' => 1],
        ];
        DB::table('comment_dislikes')->insert($commentDislikes);

        // Reply Likes
        $replyLikes = [
            ['reply_id' => 1, 'like_id' => 4],
        ];
        DB::table('reply_likes')->insert($replyLikes);

        // Reply Dislikes
        $replyDislikes = [
            ['reply_id' => 2, 'dislike_id' => 2],
        ];
        DB::table('reply_dislikes')->insert($replyDislikes);
    }
}
