use crate::db::models::*;
use diesel::prelude::*;

pub fn find_users(conn: &mut MysqlConnection) -> Result<Vec<User>, anyhow::Error> {
    use crate::schema::users::dsl::*;

    let results = conn.transaction(|conn| users.select(User::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_projects(conn: &mut MysqlConnection) -> Result<Vec<Project>, anyhow::Error> {
    use crate::schema::projects::dsl::*;

    let results = conn.transaction(|conn| projects.select(Project::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_images(conn: &mut MysqlConnection) -> Result<Vec<Image>, anyhow::Error> {
    use crate::schema::images::dsl::*;

    let results = conn.transaction(|conn| images.select(Image::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_follows(conn: &mut MysqlConnection) -> Result<Vec<Follow>, anyhow::Error> {
    use crate::schema::follows::dsl::*;

    let results = conn.transaction(|conn| follows.select(Follow::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_favourites(conn: &mut MysqlConnection) -> Result<Vec<Favourite>, anyhow::Error> {
    use crate::schema::favourites::dsl::*;

    let results = conn.transaction(|conn| favourites.select(Favourite::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_comments(conn: &mut MysqlConnection) -> Result<Vec<Comment>, anyhow::Error> {
    use crate::schema::comments::dsl::*;

    let results = conn.transaction(|conn| comments.select(Comment::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_replies(conn: &mut MysqlConnection) -> Result<Vec<Reply>, anyhow::Error> {
    use crate::schema::replies::dsl::*;

    let results = conn.transaction(|conn| replies.select(Reply::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_threads(conn: &mut MysqlConnection) -> Result<Vec<Thread>, anyhow::Error> {
    use crate::schema::threads::dsl::*;

    let results = conn.transaction(|conn| threads.select(Thread::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_likes(conn: &mut MysqlConnection) -> Result<Vec<Like>, anyhow::Error> {
    use crate::schema::likes::dsl::*;

    let results = conn.transaction(|conn| likes.select(Like::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_dislikes(conn: &mut MysqlConnection) -> Result<Vec<Dislike>, anyhow::Error> {
    use crate::schema::dislikes::dsl::*;

    let results = conn.transaction(|conn| dislikes.select(Dislike::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_comment_likes(conn: &mut MysqlConnection) -> Result<Vec<CommentLike>, anyhow::Error> {
    use crate::schema::comment_likes::dsl::*;

    let results =
        conn.transaction(|conn| comment_likes.select(CommentLike::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_comment_dislikes(
    conn: &mut MysqlConnection,
) -> Result<Vec<CommentDislike>, anyhow::Error> {
    use crate::schema::comment_dislikes::dsl::*;

    let results = conn.transaction(|conn| {
        comment_dislikes
            .select(CommentDislike::as_select())
            .load(conn)
    });

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_reply_likes(conn: &mut MysqlConnection) -> Result<Vec<ReplyLike>, anyhow::Error> {
    use crate::schema::reply_likes::dsl::*;

    let results = conn.transaction(|conn| reply_likes.select(ReplyLike::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_reply_dislikes(conn: &mut MysqlConnection) -> Result<Vec<ReplyDislike>, anyhow::Error> {
    use crate::schema::reply_dislikes::dsl::*;

    let results =
        conn.transaction(|conn| reply_dislikes.select(ReplyDislike::as_select()).load(conn));

    match results {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error.into()),
    }
}

pub fn find_user_by_id(conn: &mut MysqlConnection, in_id: i32) -> Result<User, anyhow::Error> {
    use crate::schema::users::dsl::*;

    let results = users.find(in_id).select(User::as_select()).get_result(conn);

    match results {
        Ok(user) => Ok(user),
        Err(error) => Err(error.into()),
    }
}

pub fn find_user_by_email(
    conn: &mut MysqlConnection,
    in_email: &str,
) -> Result<User, anyhow::Error> {
    use crate::schema::users::dsl::*;

    let results = users
        .select(User::as_select())
        .filter(email.eq(in_email))
        .get_result(conn);

    match results {
        Ok(user) => Ok(user),
        Err(error) => Err(error.into()),
    }
}

pub fn find_user_by_username(
    conn: &mut MysqlConnection,
    in_username: &str,
) -> Result<User, anyhow::Error> {
    use crate::schema::users::dsl::*;

    let results = users
        .filter(username.eq(in_username))
        .select(User::as_select())
        .get_result(conn);

    match results {
        Ok(user) => Ok(user),
        Err(error) => Err(error.into()),
    }
}

pub fn find_project_by_id(
    conn: &mut MysqlConnection,
    in_id: i32,
) -> Result<Project, anyhow::Error> {
    use crate::schema::projects::dsl::*;

    let project = projects
        .find(in_id)
        .select(Project::as_select())
        .get_result(conn);

    match project {
        Ok(project) => Ok(project),
        Err(error) => Err(error.into()),
    }
}

pub fn find_project_by_name(
    conn: &mut MysqlConnection,
    in_name: &str,
) -> Result<Project, anyhow::Error> {
    use crate::schema::projects::dsl::*;

    let project = projects
        .select(Project::as_select())
        .filter(name.eq(in_name))
        .get_result(conn);

    match project {
        Ok(project) => Ok(project),
        Err(error) => Err(error.into()),
    }
}

pub fn find_projects_by_user_id(
    conn: &mut MysqlConnection,
    in_user_id: i32,
) -> Result<Vec<Project>, anyhow::Error> {
    use crate::schema::projects;
    use crate::schema::users;

    let user = users::table
        .filter(users::columns::user_id.eq(in_user_id))
        .select(User::as_select())
        .get_result::<User>(conn)?;

    let projects = Project::belonging_to(&user)
        .order(projects::columns::date_created.desc())
        .select(Project::as_select())
        .get_results::<Project>(conn);

    match projects {
        Ok(projects) => Ok(projects),
        Err(error) => Err(error.into()),
    }
}

pub fn find_projects_by_user_id_udate_desc(
    conn: &mut MysqlConnection,
    in_user_id: i32,
) -> Result<Vec<Project>, anyhow::Error> {
    use crate::schema::projects;
    use crate::schema::users;

    let user = users::table
        .filter(users::columns::user_id.eq(in_user_id))
        .select(User::as_select())
        .get_result::<User>(conn)?;

    println!("User Id: {}", user.user_id);

    let projects = Project::belonging_to(&user)
        .filter(projects::columns::date_updated.is_not_null())
        .order(projects::columns::date_updated.desc())
        .select(Project::as_select())
        .get_results::<Project>(conn);

    match projects {
        Ok(projects) => {
            println!("Projects: {:?}", projects);
            Ok(projects)
        }
        Err(error) => Err(error.into()),
    }
}

pub fn find_favourites_by_user_id(
    conn: &mut MysqlConnection,
    in_user_id: i32,
) -> Result<Vec<Favourite>, anyhow::Error> {
    use crate::schema::users;

    let user = users::table
        .filter(users::columns::user_id.eq(in_user_id))
        .select(User::as_select())
        .get_result::<User>(conn)?;

    let favourites = Favourite::belonging_to(&user)
        .select(Favourite::as_select())
        .get_results::<Favourite>(conn);

    match favourites {
        Ok(favourites) => {
            println!("Favourites: {:?}", favourites);
            Ok(favourites)
        }
        Err(error) => Err(error.into()),
    }
}

pub fn find_favourite_by_ids(conn: &mut MysqlConnection, in_user_id: i32, in_project_id: i32) -> Result<Favourite, anyhow::Error> {
    use crate::schema::favourites::dsl::*;

    let result = favourites
        .select(Favourite::as_select())
        .filter(user_id.eq(in_user_id).and(project_id.eq(in_project_id)))
        .first::<Favourite>(conn);

    match result {
        Ok(result) => {
            println!("Favourites: {:?}", result);
            Ok(result)
        }
        Err(error) => Err(error.into()),
    }
}