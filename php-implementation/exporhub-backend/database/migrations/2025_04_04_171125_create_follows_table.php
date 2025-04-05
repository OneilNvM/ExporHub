<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('follows', function (Blueprint $table) {
            $table->id('follow_id');
            $table->foreignId('follower')->constrained(
                table: 'users', column: 'user_id', indexName: 'follows_follower_user_id_foreign'
            );
            $table->foreignId('following')->constrained(
                table: 'users', column: 'user_id', indexName: 'follows_following_user_id_foreign'
            );;
            $table->dateTime('date_followed', 6)->useCurrent();
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('follows');
    }
};
