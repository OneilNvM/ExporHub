use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use expor_hub_server::db::db_actions::{connection::establish_connection, selects::*};

fn retrieve_all_rows_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("all-rows-linear-sampling");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("retrieve_all_rows", |b| {
        b.iter(|| {
            let conn = &mut establish_connection();
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

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = retrieve_all_rows_benchmark
}

criterion_main!(benches);
