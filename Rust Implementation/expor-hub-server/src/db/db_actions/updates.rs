use std::collections::HashMap;

use diesel::prelude::*;
use crate::db::models::*;

pub fn update_user(conn: &mut MysqlConnection, in_user_id: i32, values_map: HashMap<&str, Option<&str>>) -> Result<Option<User>, anyhow::Error> {
    use crate::schema::users::dsl::*;

    let user = conn.transaction(|conn| {
        for (key, value) in values_map {
            match key {
                "username" => {
                    diesel::update(users.find(in_user_id))
                        .set(username.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                },
                "email" => {
                    diesel::update(users.find(in_user_id))
                        .set(email.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                },
                "password" => {
                    diesel::update(users.find(in_user_id))
                        .set(password.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                },
                "bio" => {
                    diesel::update(users.find(in_user_id))
                        .set(bio.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                },
                "profile_img" => {
                    diesel::update(users.find(in_user_id))
                        .set(profile_img.eq(value.unwrap().to_owned()))
                        .execute(conn)?;
                },
                "followers" => {
                    let num: i32 = value.unwrap().parse().unwrap();

                    diesel::update(users.find(in_user_id))
                        .set(followers.eq(num))
                        .execute(conn)?;
                },
                _ => eprintln!("No column named {key}")
            }
        }

        users.find(in_user_id).select(User::as_select()).first(conn).optional()
    });

    match user {
        Ok(user) => Ok(user),
        Err(error) => Err(error.into())
    }
}