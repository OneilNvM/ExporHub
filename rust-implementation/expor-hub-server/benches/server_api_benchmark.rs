use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use expor_hub_server::db::db_actions::{connection::establish_connection, selects::*};

fn retrieve_all_rows_benchmark(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("all-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_all_rows", |b| {
        b.iter(|| {
            let _users = find_users(conn);
            let _projects = find_projects(conn);
            let _images = find_images(conn);
            let _follows = find_follows(conn);
            let _favourites = find_favourites(conn);
            let _comments = find_comments(conn);
            let _replies = find_replies(conn);
            let _threads = find_threads(conn);
            let _likes = find_likes(conn);
            let _dislikes = find_dislikes(conn);
            let _comment_likes = find_comment_likes(conn);
            let _comment_dislikes = find_comment_dislikes(conn);
            let _reply_likes = find_reply_likes(conn);
            let _reply_dislikes = find_reply_dislikes(conn);
        })
    });
    group.finish();
}

fn retrieve_user_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("user-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_user_rows", |b| {
        b.iter(|| {
            let _users = find_users(conn);
        })
    });
    group.finish();
}

fn retrieve_project_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("project-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_project_rows", |b| {
        b.iter(|| {
            let _projects = find_projects(conn);
        })
    });
    group.finish();
}

fn retrieve_image_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("image-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_image_rows", |b| {
        b.iter(|| {
            let _images = find_images(conn);
        })
    });
    group.finish();
}

fn retrieve_follow_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("follow-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_follow_rows", |b| {
        b.iter(|| {
            let _follows = find_follows(conn);
        })
    });
    group.finish();
}

fn retrieve_favourite_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("favourite-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_favourite_rows", |b| {
        b.iter(|| {
            let _favourites = find_favourites(conn);
        })
    });
    group.finish();
}

fn retrieve_comment_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("comment-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_comment_rows", |b| {
        b.iter(|| {
            let _comments = find_comments(conn);
        })
    });
    group.finish();
}

fn retrieve_reply_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("reply-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_reply_rows", |b| {
        b.iter(|| {
            let _replies = find_replies(conn);
        })
    });
    group.finish();
}

fn retrieve_thread_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("thread-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_thread_rows", |b| {
        b.iter(|| {
            let _threads = find_threads(conn);
        })
    });
    group.finish();
}

fn retrieve_like_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("like-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_like_rows", |b| {
        b.iter(|| {
            let _likes = find_likes(conn);
        })
    });
    group.finish();
}

fn retrieve_dislike_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("dislike-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_dislike_rows", |b| {
        b.iter(|| {
            let _dislikes = find_dislikes(conn);
        })
    });
    group.finish();
}

fn retrieve_comment_like_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("comment_like-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_comment_like_rows", |b| {
        b.iter(|| {
            let _comment_likes = find_comment_likes(conn);
        })
    });
    group.finish();
}

fn retrieve_comment_dislike_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("comment_dislike-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_comment_dislike_rows", |b| {
        b.iter(|| {
            let _comment_dislikes = find_comment_dislikes(conn);
        })
    });
    group.finish();
}

fn retrieve_reply_like_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("reply_like-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_reply_like_rows", |b| {
        b.iter(|| {
            let _reply_likes = find_reply_likes(conn);
        })
    });
    group.finish();
}

fn retrieve_reply_dislike_rows(c: &mut Criterion) {
    let conn = &mut establish_connection();
    let mut group = c.benchmark_group("reply_dislike-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_reply_dislike_rows", |b| {
        b.iter(|| {
            let _reply_dislikes = find_reply_dislikes(conn);
        })
    });
    group.finish();
}

criterion_group!{
    name = all_rows_bench;
    config = Criterion::default().sample_size(20);
    targets = retrieve_all_rows_benchmark
}

criterion_group! {
    name = group_1_bench;
    config = Criterion::default().sample_size(20);
    targets =
        retrieve_user_rows,
        retrieve_project_rows,
        retrieve_image_rows,
}

criterion_group! {
    name = group_2_bench;
    config = Criterion::default().sample_size(20);
    targets =
        retrieve_follow_rows,
        retrieve_favourite_rows,
        retrieve_comment_rows,
}

criterion_group! {
    name = group_3_bench;
    config = Criterion::default().sample_size(20);
    targets =
        retrieve_reply_rows,
        retrieve_thread_rows,
        retrieve_like_rows,
}

criterion_group! {
    name = group_4_bench;
    config = Criterion::default().sample_size(20);
    targets =
        retrieve_dislike_rows,
        retrieve_comment_like_rows,
        retrieve_comment_dislike_rows,
}
criterion_group! {
    name = group_5_bench;
    config = Criterion::default().sample_size(20);
    targets =
        retrieve_reply_like_rows,
        retrieve_reply_dislike_rows
}

criterion_main!(all_rows_bench, group_1_bench, group_2_bench, group_3_bench, group_4_bench, group_5_bench);
