<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;
use Illuminate\Support\Facades\DB;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('comments', function (Blueprint $table) {
            $table->id('comment_id');
            $table->string('text');
            $table->date('date')->default(DB::raw('(curdate())'));
            $table->foreignId('user_id')->constrained(
                table: 'users', column: 'user_id'
            );
            $table->foreignId('project_id')->constrained(
                table: 'projects', column: 'project_id'
            );
            $table->bigInteger('replies')->default(0);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('comments');
    }
};
