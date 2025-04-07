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
        Schema::create('users', function (Blueprint $table) {
            $table->id('user_id');
            $table->string('username', 100)->unique('username');
            $table->string('email', 255)->unique('email');
            $table->string('password', 255);
            $table->string('bio', 5000)->nullable();
            $table->string('profile_img', 255)->nullable();
            $table->bigInteger('followers')->default(0);
            $table->dateTimeTz('date_created', 6)->useCurrent();
            $table->fullText(['username', 'bio'], 'user_search_idx');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('users');
    }
};
