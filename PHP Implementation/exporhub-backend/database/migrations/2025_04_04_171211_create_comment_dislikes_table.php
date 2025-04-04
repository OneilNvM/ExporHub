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
        Schema::create('comment_dislikes', function (Blueprint $table) {
            $table->id();
            $table->foreignId('comment_id')->constrained(
                table: 'comments', column: 'comment_id'
            );
            $table->foreignId('dislike_id')->constrained(
                table: 'dislikes', column: 'dislike_id'
            );
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('comment_dislikes');
    }
};
