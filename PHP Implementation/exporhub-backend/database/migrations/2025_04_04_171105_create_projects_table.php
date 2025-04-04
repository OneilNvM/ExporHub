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
        Schema::create('projects', function (Blueprint $table) {
            $table->id('project_id');
            $table->string('name', 255)->unique('project_name');
            $table->string('description', 5000);
            $table->bigInteger('favourites')->default(0);
            $table->foreignId('user_id')->constrained(
                table: 'users', column: 'user_id'
            );
            $table->dateTime('date_created', 6)->useCurrent();
            $table->dateTime('date_updated', 6)->nullable();
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('projects');
    }
};
