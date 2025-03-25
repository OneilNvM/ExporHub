use crate::db::db_actions::connection::establish_connection;

// Unit Tests for Inserts
#[test]
fn should_create_account() {
    use crate::db::db_actions::inserts::insert_user;

    let mut conn = establish_connection();

    let user = insert_user(&mut conn, "Alisson", "alisson@gmail.com", "alissonlistens");

    if let Err(error) = &user {
        eprintln!("Error: {error}")
    } else {
        println!("{user:#?}")
    }

    assert!(user.is_ok() && user.unwrap().is_some())
}

#[test]
fn should_create_project() {
    use crate::db::db_actions::inserts::insert_project;

    let mut conn = establish_connection();

    let project = insert_project(&mut conn, "New Project", "Test description for project", 1);

    if let Err(error) = &project {
        eprintln!("Error: {error}")
    } else {
        println!("{project:#?}")
    }

    assert!(project.is_ok() && project.unwrap().is_some())
}

#[test]
fn should_create_profile_image() {
    use crate::db::db_actions::inserts::insert_profile_image;

    let mut conn = establish_connection();

    let image = insert_profile_image(&mut conn, "path/to/image", 0);

    if let Err(error) = &image {
        eprintln!("Error: {error}")
    } else {
        println!("{image:#?}")
    }

    assert!(image.is_ok() && image.unwrap().is_some())
}

#[test]
fn should_create_project_image() {
    use crate::db::db_actions::inserts::insert_project_image;

    let mut conn = establish_connection();

    let image = insert_project_image(&mut conn, "some/path/to/image", 0, 0);

    if let Err(error) = &image {
        eprintln!("Error: {error}")
    } else {
        println!("{image:#?}")
    }

    assert!(image.is_ok() && image.unwrap().is_some())
}

#[test]
fn should_create_follow() {
    use crate::db::db_actions::inserts::insert_follow;

    let mut conn = establish_connection();

    let follow = insert_follow(&mut conn, 0, 0);

    if let Err(error) = &follow {
        eprintln!("Error: {error}")
    } else {
        println!("{follow:#?}")
    }

    assert!(follow.is_ok() && follow.unwrap().is_some())
}

#[test]
fn should_create_favourite() {
    use crate::db::db_actions::inserts::insert_favourite;

    let mut conn = establish_connection();

    let favourite = insert_favourite(&mut conn, 0, 0);

    if let Err(error) = &favourite {
        eprintln!("Error: {error}")
    } else {
        println!("{favourite:#?}")
    }

    assert!(favourite.is_ok() && favourite.unwrap().is_some())
}

#[test]
fn should_create_comment() {
    use crate::db::db_actions::inserts::insert_comment;

    let mut conn = establish_connection();

    let comment = insert_comment(&mut conn, "Some text", 0, 0);

    if let Err(error) = &comment {
        eprintln!("Error: {error}")
    } else {
        println!("{comment:#?}")
    }

    assert!(comment.is_ok() && comment.unwrap().is_some())
}

#[test]
fn should_create_reply() {
    use crate::db::db_actions::inserts::insert_reply;

    let mut conn = establish_connection();

    let reply = insert_reply(&mut conn, "Some text", 0);

    if let Err(error) = &reply {
        eprintln!("Error: {error}")
    } else {
        println!("{reply:#?}")
    }

    assert!(reply.is_ok() && reply.unwrap().is_some())
}

#[test]
fn should_create_thread() {
    use crate::db::db_actions::inserts::insert_thread;

    let mut conn = establish_connection();

    let thread = insert_thread(&mut conn, 0, 0);

    if let Err(error) = &thread {
        eprintln!("Error: {error}")
    } else {
        println!("{thread:#?}")
    }

    assert!(thread.is_ok() && thread.unwrap().is_some())
}

#[test]
fn should_create_like() {
    use crate::db::db_actions::inserts::insert_like;

    let mut conn = establish_connection();

    let like = insert_like(&mut conn);

    if let Err(error) = &like {
        eprintln!("Error: {error}")
    } else {
        println!("{like:#?}")
    }

    assert!(like.is_ok() && like.unwrap().is_some())
}

#[test]
fn should_create_dislike() {
    use crate::db::db_actions::inserts::insert_dislike;

    let mut conn = establish_connection();

    let dislike = insert_dislike(&mut conn);

    if let Err(error) = &dislike {
        eprintln!("Error: {error}")
    } else {
        println!("{dislike:#?}")
    }

    assert!(dislike.is_ok() && dislike.unwrap().is_some())
}

#[test]
fn should_create_comment_like() {
    use crate::db::db_actions::inserts::insert_comment_like;

    let mut conn = establish_connection();

    let comment_like = insert_comment_like(&mut conn, 0, 0);

    if let Err(error) = &comment_like {
        eprintln!("Error: {error}")
    } else {
        println!("{comment_like:#?}")
    }

    assert!(comment_like.is_ok() && comment_like.unwrap().is_some())
}

#[test]
fn should_create_comment_dislike() {
    use crate::db::db_actions::inserts::insert_comment_dislike;

    let mut conn = establish_connection();

    let comment_dislike = insert_comment_dislike(&mut conn, 0, 0);

    if let Err(error) = &comment_dislike {
        eprintln!("Error: {error}")
    } else {
        println!("{comment_dislike:#?}")
    }

    assert!(comment_dislike.is_ok() && comment_dislike.unwrap().is_some())
}

#[test]
fn should_create_reply_like() {
    use crate::db::db_actions::inserts::insert_reply_like;

    let mut conn = establish_connection();

    let reply_like = insert_reply_like(&mut conn, 0, 0);

    if let Err(error) = &reply_like {
        eprintln!("Error: {error}")
    } else {
        println!("{reply_like:#?}")
    }

    assert!(reply_like.is_ok() && reply_like.unwrap().is_some())
}

#[test]
fn should_create_reply_dislike() {
    use crate::db::db_actions::inserts::insert_reply_dislike;

    let mut conn = establish_connection();

    let reply_dislike = insert_reply_dislike(&mut conn, 0, 0);

    if let Err(error) = &reply_dislike {
        eprintln!("Error: {error}")
    } else {
        println!("{reply_dislike:#?}")
    }

    assert!(reply_dislike.is_ok() && reply_dislike.unwrap().is_some())
}

// Unit Tests for Deletes

#[test]
#[should_panic]
fn should_delete_user_by_id() {
    use crate::db::db_actions::deletes::delete_user_by_id;

    let mut conn = establish_connection();

    let result = delete_user_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_user_by_username() {
    use crate::db::db_actions::deletes::delete_user_by_username;

    let mut conn = establish_connection();

    let result = delete_user_by_username(&mut conn, "Test");

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_project_by_id() {
    use crate::db::db_actions::deletes::delete_user_by_id;

    let mut conn = establish_connection();

    let result = delete_user_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_project_by_name() {
    use crate::db::db_actions::deletes::delete_project_by_name;

    let mut conn = establish_connection();

    let result = delete_project_by_name(&mut conn, "Test");

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_image_by_id() {
    use crate::db::db_actions::deletes::delete_image_by_id;

    let mut conn = establish_connection();

    let result = delete_image_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_image_by_file_name() {
    use crate::db::db_actions::deletes::delete_image_by_file_name;

    let mut conn = establish_connection();

    let result = delete_image_by_file_name(&mut conn, "path/to/image");

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_favourite_by_id() {
    use crate::db::db_actions::deletes::delete_favourite_by_id;

    let mut conn = establish_connection();

    let result = delete_favourite_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_favourite_by_ids() {
    use crate::db::db_actions::deletes::delete_favourite_by_ids;

    let mut conn = establish_connection();

    let result = delete_favourite_by_ids(&mut conn, 0, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_follow_by_id() {
    use crate::db::db_actions::deletes::delete_follow_by_id;

    let mut conn = establish_connection();

    let result = delete_follow_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_follow_by_ids() {
    use crate::db::db_actions::deletes::delete_follow_by_ids;

    let mut conn = establish_connection();

    let result = delete_follow_by_ids(&mut conn, 0, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_comment_by_id() {
    use crate::db::db_actions::deletes::delete_comment_by_id;

    let mut conn = establish_connection();

    let result = delete_comment_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_reply_by_id() {
    use crate::db::db_actions::deletes::delete_reply_by_id;

    let mut conn = establish_connection();

    let result = delete_reply_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_thread_by_id() {
    use crate::db::db_actions::deletes::delete_thread_by_id;

    let mut conn = establish_connection();

    let result = delete_thread_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_thread_by_ids() {
    use crate::db::db_actions::deletes::delete_thread_by_ids;

    let mut conn = establish_connection();

    let result = delete_thread_by_ids(&mut conn, 0, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_like_by_id() {
    use crate::db::db_actions::deletes::delete_like_by_id;

    let mut conn = establish_connection();

    let result = delete_like_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_dislike_by_id() {
    use crate::db::db_actions::deletes::delete_dislike_by_id;

    let mut conn = establish_connection();

    let result = delete_dislike_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_comment_like_by_id() {
    use crate::db::db_actions::deletes::delete_comment_like_by_id;

    let mut conn = establish_connection();

    let result = delete_comment_like_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_comment_like_by_ids() {
    use crate::db::db_actions::deletes::delete_comment_like_by_ids;

    let mut conn = establish_connection();

    let result = delete_comment_like_by_ids(&mut conn, 0, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_comment_dislike_by_id() {
    use crate::db::db_actions::deletes::delete_comment_dislike_by_id;

    let mut conn = establish_connection();

    let result = delete_comment_dislike_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_comment_dislike_by_ids() {
    use crate::db::db_actions::deletes::delete_comment_dislike_by_ids;

    let mut conn = establish_connection();

    let result = delete_comment_dislike_by_ids(&mut conn, 0, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_reply_like_by_id() {
    use crate::db::db_actions::deletes::delete_reply_like_by_id;

    let mut conn = establish_connection();

    let result = delete_reply_like_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_reply_like_by_ids() {
    use crate::db::db_actions::deletes::delete_reply_like_by_ids;

    let mut conn = establish_connection();

    let result = delete_reply_like_by_ids(&mut conn, 0, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_reply_dislike_by_id() {
    use crate::db::db_actions::deletes::delete_reply_dislike_by_id;

    let mut conn = establish_connection();

    let result = delete_reply_dislike_by_id(&mut conn, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

#[test]
#[should_panic]
fn should_delete_reply_dislike_by_ids() {
    use crate::db::db_actions::deletes::delete_reply_dislike_by_ids;

    let mut conn = establish_connection();

    let result = delete_reply_dislike_by_ids(&mut conn, 0, 0);

    if let Err(error) = &result {
        eprintln!("Error: {error}")
    } else {
        println!("Number of rows deleted: {}", result.as_ref().unwrap())
    }

    assert!(result.is_ok())
}

// Unit Tests for Selects
#[test]
fn should_return_user_by_id() {
    use crate::db::db_actions::selects::find_user_by_id;

    let mut conn = establish_connection();

    let user = find_user_by_id(&mut conn, 1);

    if let Err(error) = &user {
        eprintln!("Error: {error}")
    } else {
        println!("{user:#?}")
    }

    assert!(user.is_ok())
}

#[test]
fn should_return_user_by_email() {
    use crate::db::db_actions::selects::find_user_by_email;

    let mut conn = establish_connection();

    let user = find_user_by_email(&mut conn, "alisson@gmail.com");

    if let Err(error) = &user {
        eprintln!("Error: {error}")
    } else {
        println!("{user:#?}")
    }

    assert!(user.is_ok())
}

#[test]
fn should_return_user_by_username() {
    use crate::db::db_actions::selects::find_user_by_username;

    let mut conn = establish_connection();

    let user = find_user_by_username(&mut conn, "whatever");

    if let Err(error) = &user {
        eprintln!("Error: {error}")
    } else {
        println!("{user:#?}")
    }

    assert!(user.is_ok())
}

#[test]
#[should_panic]
fn should_not_return_project_by_id() {
    use crate::db::db_actions::selects::find_project_by_id;

    let mut conn = establish_connection();

    let project = find_project_by_id(&mut conn, 0);

    if let Err(error) = &project {
        eprintln!("Error: {error}")
    } else {
        println!("{project:#?}")
    }

    assert!(project.is_ok())
}

#[test]
#[should_panic]
fn should_not_return_project_by_name() {
    use crate::db::db_actions::selects::find_project_by_name;

    let mut conn = establish_connection();

    let project = find_project_by_name(&mut conn, "New Project");

    if let Err(error) = &project {
        eprintln!("Error: {error}")
    } else {
        println!("{project:#?}")
    }

    assert!(project.is_ok())
}

// Unit Tests for Updates

#[test]
fn should_update_user() {
    use crate::db::db_actions::updates::update_user;
    use std::collections::HashMap;

    let conn = &mut establish_connection();

    let mut values: HashMap<&str, Option<&str>> = HashMap::new();

    values.insert("username", Some("OneilNvM"));

    let user = update_user(conn, 1, values);

    if let Err(error) = &user {
        eprintln!("Error: {error}")
    } else {
        println!("{user:#?}")
    }

    assert!(user.is_ok() && user.unwrap().is_some())
}

// Unit Tests for REST API Routes