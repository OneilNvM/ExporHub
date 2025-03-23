use crate::db::db_actions::connection::establish_connection;

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

    assert!(user.is_ok())
}

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

    assert!(project.is_ok())
}