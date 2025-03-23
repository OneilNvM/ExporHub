use crate::db::actions::establish_connection;

#[test]
fn should_create_account() {
    use crate::db::actions::insert_user;

    let mut conn = establish_connection();

    let username = "Alisson";
    let email = "alisson@gmail.com";
    let password = "alissonlistens";
    let user = insert_user(&mut conn, username, email, password);

    if let Err(error) = &user {
        eprintln!("Error: {error}")
    } else {
        println!("{user:#?}")
    }

    assert!(user.is_ok())
}

#[test]
fn should_return_user_by_id() {
    use crate::db::actions::find_user_by_id;

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
    use crate::db::actions::find_user_by_email;

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
    use crate::db::actions::find_user_by_username;

    let mut conn = establish_connection();

    let user = find_user_by_username(&mut conn, "whatever");

    if let Err(error) = &user {
        eprintln!("Error: {error}")
    } else {
        println!("{user:#?}")
    }

    assert!(user.is_ok())
}