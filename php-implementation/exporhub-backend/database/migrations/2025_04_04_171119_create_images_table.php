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
        Schema::create('images', function (Blueprint $table) {
            $table->id('image_id');
            $table->string('file_path', 255)->unique('image_path');
            $table->foreignId('user_id')->nullable()->constrained(
                table: 'users', column: 'user_id'
            );
            $table->foreignId('project_id')->nullable()->constrained(
                table: 'projects', column: 'project_id'
            );
            $table->dateTime('date_uploaded', 6)->useCurrent();
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('images');
    }
};
