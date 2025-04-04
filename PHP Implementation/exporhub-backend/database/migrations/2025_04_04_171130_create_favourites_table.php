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
        Schema::create('favourites', function (Blueprint $table) {
            $table->id('favourite_id');
            $table->foreignId('user_id')->constrained(
                table: 'users', column: 'user_id'
            );
            $table->foreignId('project_id')->constrained(
                table: 'projects', column: 'project_id'
            );
            $table->dateTime('date_favourited', 6)->useCurrent();
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('favourites');
    }
};
