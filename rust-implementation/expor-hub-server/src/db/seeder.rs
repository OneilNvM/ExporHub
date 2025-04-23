use diesel::prelude::*;
use chrono::Local;
use sha2::{Digest, Sha256};

use crate::db::db_actions::connection::establish_connection;

use super::models::*;

// Make sure to run 'cargo run -- db-seed' in order to insert the data

pub fn run() -> Result<(), diesel::result::Error> {
    use crate::schema::comment_dislikes;
    use crate::schema::comment_likes;
    use crate::schema::comments;
    use crate::schema::dislikes;
    use crate::schema::favourites;
    use crate::schema::follows;
    use crate::schema::images;
    use crate::schema::likes;
    use crate::schema::projects;
    use crate::schema::replies;
    use crate::schema::reply_dislikes;
    use crate::schema::reply_likes;
    use crate::schema::threads;
    use crate::schema::users;

    let conn = &mut establish_connection();

    // --- Insert Users ---
    let mut hasher_1 = Sha256::new();
    hasher_1.update("hashed_password_1".as_bytes());
  
    let result_1 = hasher_1.finalize();
    let password_1 = hex::encode(result_1);
  
    let mut hasher_2 = Sha256::new();
    hasher_2.update("hashed_password_2".as_bytes());
  
    let result_2 = hasher_2.finalize();
    let password_2 = hex::encode(result_2);
  
    let mut hasher_3 = Sha256::new();
    hasher_3.update("hashed_password_3".as_bytes());
  
    let result_3 = hasher_3.finalize();
    let password_3 = hex::encode(result_3);
  
    let mut hasher_4 = Sha256::new();
    hasher_4.update("hashed_password_4".as_bytes());
  
    let result_4 = hasher_4.finalize();
    let password_4 = hex::encode(result_4);
  
    let mut hasher_5 = Sha256::new();
    hasher_5.update("hashed_password_5".as_bytes());
  
    let result_5 = hasher_5.finalize();
    let password_5 = hex::encode(result_5);
  
    let mut hasher_6 = Sha256::new();
    hasher_6.update("hashed_password_6".as_bytes());
  
    let result_6 = hasher_6.finalize();
    let password_6 = hex::encode(result_6);
  
    let mut hasher_7 = Sha256::new();
    hasher_7.update("hashed_password_7".as_bytes());
  
    let result_7 = hasher_7.finalize();
    let password_7 = hex::encode(result_7);
  
    let mut hasher_8 = Sha256::new();
    hasher_8.update("hashed_password_8".as_bytes());
  
    let result_8 = hasher_8.finalize();
    let password_8 = hex::encode(result_8);
  
    let mut hasher_9 = Sha256::new();
    hasher_9.update("hashed_password_9".as_bytes());
  
    let result_9 = hasher_9.finalize();
    let password_9 = hex::encode(result_9);
  
    let mut hasher_10 = Sha256::new();
    hasher_10.update("hashed_password_10".as_bytes());
  
    let result_10 = hasher_10.finalize();
    let password_10 = hex::encode(result_10);
  
    let mut hasher_11 = Sha256::new();
    hasher_11.update("hashed_password_11".as_bytes());
  
    let result_11 = hasher_11.finalize();
    let password_11 = hex::encode(result_11);
  
    let mut hasher_12 = Sha256::new();
    hasher_12.update("hashed_password_12".as_bytes());
  
    let result_12 = hasher_12.finalize();
    let password_12 = hex::encode(result_12);
  
    let mut hasher_13 = Sha256::new();
    hasher_13.update("hashed_password_13".as_bytes());
  
    let result_13 = hasher_13.finalize();
    let password_13 = hex::encode(result_13);
  
    let mut hasher_14 = Sha256::new();
    hasher_14.update("hashed_password_14".as_bytes());
  
    let result_14 = hasher_14.finalize();
    let password_14 = hex::encode(result_14);
  
    let mut hasher_15 = Sha256::new();
    hasher_15.update("hashed_password_15".as_bytes());
  
    let result_15 = hasher_15.finalize();
    let password_15 = hex::encode(result_15);
  
    let mut hasher_16 = Sha256::new();
    hasher_16.update("hashed_password_16".as_bytes());
  
    let result_16 = hasher_16.finalize();
    let password_16 = hex::encode(result_16);
  
    let mut hasher_17 = Sha256::new();
    hasher_17.update("hashed_password_17".as_bytes());
  
    let result_17 = hasher_17.finalize();
    let password_17 = hex::encode(result_17);
  
    let mut hasher_18 = Sha256::new();
    hasher_18.update("hashed_password_18".as_bytes());
  
    let result_18 = hasher_18.finalize();
    let password_18 = hex::encode(result_18);

    let mut hasher_19 = Sha256::new();
    hasher_19.update("hashed_password_19".as_bytes());
  
    let result_19 = hasher_19.finalize();
    let password_19 = hex::encode(result_19);

    let mut hasher_20 = Sha256::new();
    hasher_20.update("hashed_password_20".as_bytes());
  
    let result_20 = hasher_20.finalize();
    let password_20 = hex::encode(result_20);

    let new_users_data = vec![
        NewUser::new("user_1", "user.1@example.com", &password_1),
        NewUser::new("user_2", "user.2@example.com", &password_2),
        NewUser::new("user_3", "user.3@example.com", &password_3),
        NewUser::new("user_4", "user.4@example.com", &password_4),
        NewUser::new("user_5", "user.5@example.com", &password_5),
        NewUser::new("user_6", "user.6@example.com", &password_6),
        NewUser::new("user_7", "user.7@example.com", &password_7),
        NewUser::new("user_8", "user.8@example.com", &password_8),
        NewUser::new("user_9", "user.9@example.com", &password_9),
        NewUser::new("user_10", "user.10@example.com", &password_10),
        NewUser::new("user_11", "user.11@example.com", &password_11),
        NewUser::new("user_12", "user.12@example.com", &password_12),
        NewUser::new("user_13", "user.13@example.com", &password_13),
        NewUser::new("user_14", "user.14@example.com", &password_14),
        NewUser::new("user_15", "user.15@example.com", &password_15),
        NewUser::new("user_16", "user.16@example.com", &password_16),
        NewUser::new("user_17", "user.17@example.com", &password_17),
        NewUser::new("user_18", "user.18@example.com", &password_18),
        NewUser::new("user_19", "user.19@example.com", &password_19),
        NewUser::new("user_20", "user.20@example.com", &password_20),
    ];

    diesel::insert_into(users::table)
        .values(&new_users_data)
        .execute(conn)?;

        let user_1 = users::table
        .filter(users::columns::username.eq("user_1"))
        .first::<User>(conn)?;
    
      let user_2 = users::table
        .filter(users::columns::username.eq("user_2"))
        .first::<User>(conn)?;
    
      let user_3 = users::table
        .filter(users::columns::username.eq("user_3"))
        .first::<User>(conn)?;
    
      let user_4 = users::table
        .filter(users::columns::username.eq("user_4"))
        .first::<User>(conn)?;
    
      let user_5 = users::table
        .filter(users::columns::username.eq("user_5"))
        .first::<User>(conn)?;
    
      let user_6 = users::table
        .filter(users::columns::username.eq("user_6"))
        .first::<User>(conn)?;
    
      let user_7 = users::table
        .filter(users::columns::username.eq("user_7"))
        .first::<User>(conn)?;
    
      let user_8 = users::table
        .filter(users::columns::username.eq("user_8"))
        .first::<User>(conn)?;
    
      let user_9 = users::table
        .filter(users::columns::username.eq("user_9"))
        .first::<User>(conn)?;
    
      let user_10 = users::table
        .filter(users::columns::username.eq("user_10"))
        .first::<User>(conn)?;
    
      let user_11 = users::table
        .filter(users::columns::username.eq("user_11"))
        .first::<User>(conn)?;
    
      let user_12 = users::table
        .filter(users::columns::username.eq("user_12"))
        .first::<User>(conn)?;
    
      let user_13 = users::table
        .filter(users::columns::username.eq("user_13"))
        .first::<User>(conn)?;
    
      let user_14 = users::table
        .filter(users::columns::username.eq("user_14"))
        .first::<User>(conn)?;
    
      let user_15 = users::table
        .filter(users::columns::username.eq("user_15"))
        .first::<User>(conn)?;
    
      let user_16 = users::table
        .filter(users::columns::username.eq("user_16"))
        .first::<User>(conn)?;
    
      let user_17 = users::table
        .filter(users::columns::username.eq("user_17"))
        .first::<User>(conn)?;
    
      let user_18 = users::table
        .filter(users::columns::username.eq("user_18"))
        .first::<User>(conn)?;
    
      let user_19 = users::table
        .filter(users::columns::username.eq("user_19"))
        .first::<User>(conn)?;
    
      let user_20 = users::table
        .filter(users::columns::username.eq("user_20"))
        .first::<User>(conn)?;

    // --- Insert Projects ---
    let new_projects_data = vec![
      NewProject::new(
        "Rust Web API",
        "A RESTful API built with Rust and Actix-web.",
        user_1.user_id,
      ),
      NewProject::new(
        "Machine Learning Model",
        "Training a neural network for image classification.",
        user_2.user_id,
      ),
      NewProject::new(
        "Container Orchestration",
        "Setting up and managing a Kubernetes cluster.",
        user_3.user_id,
      ),
      NewProject::new(
        "Cloud Infrastructure Deployment",
        "Provisioning resources on AWS using Terraform.",
        user_4.user_id,
      ),
      NewProject::new(
        "Data Analysis Pipeline",
        "Building an ETL pipeline with Python and Pandas.",
        user_5.user_id,
      ),
      NewProject::new(
        "Mobile App Development (React Native)",
        "Creating a cross-platform mobile application.",
        user_6.user_id,
      ),
      NewProject::new(
        "Blockchain Implementation",
        "Developing a simple blockchain prototype.",
        user_7.user_id,
      ),
      NewProject::new(
        "IoT Sensor Network",
        "Setting up and collecting data from IoT devices.",
        user_8.user_id,
      ),
      NewProject::new(
        "Cybersecurity Audit",
        "Performing a security assessment of a web application.",
        user_9.user_id,
      ),
      NewProject::new(
        "Game Development (Unity)",
        "Building a 2D platformer game.",
        user_10.user_id,
      ),
      NewProject::new(
        "Natural Language Processing",
        "Implementing sentiment analysis on text data.",
        user_11.user_id,
      ),
      NewProject::new(
        "Database Migration",
        "Migrating a PostgreSQL database to a new server.",
        user_12.user_id,
      ),
      NewProject::new(
        "Front-end Framework (Vue.js)",
        "Developing a user interface with Vue.js.",
        user_13.user_id,
      ),
      NewProject::new(
        "DevOps Automation",
        "Automating deployment processes with CI/CD.",
        user_14.user_id,
      ),
      NewProject::new(
        "Computer Vision Project",
        "Implementing object detection in video streams.",
        user_15.user_id,
      ),
      NewProject::new(
        "GraphQL API",
        "Building a GraphQL API for data fetching.",
        user_16.user_id,
      ),
      NewProject::new(
        "Serverless Function Development",
        "Creating AWS Lambda functions for backend logic.",
        user_17.user_id,
      ),
      NewProject::new(
        "Real-time Communication",
        "Implementing a chat application using WebSockets.",
        user_18.user_id,
      ),
      NewProject::new(
        "Big Data Processing (Spark)",
        "Analyzing large datasets using Apache Spark.",
        user_19.user_id,
      ),
      NewProject::new(
        "Robotics Software Development",
        "Writing software for a robotic arm.",
        user_20.user_id,
      ),
    ];

    diesel::insert_into(projects::table)
        .values(&new_projects_data)
        .execute(conn)?;

        let project_1 = projects::table
        .filter(projects::columns::name.eq("Rust Web API"))
        .first::<Project>(conn)?;
    
      let project_2 = projects::table
        .filter(projects::columns::name.eq("Machine Learning Model"))
        .first::<Project>(conn)?;
    
      let project_3 = projects::table
        .filter(projects::columns::name.eq("Container Orchestration"))
        .first::<Project>(conn)?;
    
      let project_4 = projects::table
        .filter(projects::columns::name.eq("Cloud Infrastructure Deployment"))
        .first::<Project>(conn)?;
    
      let project_5 = projects::table
        .filter(projects::columns::name.eq("Data Analysis Pipeline"))
        .first::<Project>(conn)?;
    
      let project_6 = projects::table
        .filter(projects::columns::name.eq("Mobile App Development (React Native)"))
        .first::<Project>(conn)?;
    
      let project_7 = projects::table
        .filter(projects::columns::name.eq("Blockchain Implementation"))
        .first::<Project>(conn)?;
    
      let project_8 = projects::table
        .filter(projects::columns::name.eq("IoT Sensor Network"))
        .first::<Project>(conn)?;
    
      let project_9 = projects::table
        .filter(projects::columns::name.eq("Cybersecurity Audit"))
        .first::<Project>(conn)?;
    
      let project_10 = projects::table
        .filter(projects::columns::name.eq("Game Development (Unity)"))
        .first::<Project>(conn)?;
    
      let project_11 = projects::table
        .filter(projects::columns::name.eq("Natural Language Processing"))
        .first::<Project>(conn)?;
    
      let project_12 = projects::table
        .filter(projects::columns::name.eq("Database Migration"))
        .first::<Project>(conn)?;
    
      let project_13 = projects::table
        .filter(projects::columns::name.eq("Front-end Framework (Vue.js)"))
        .first::<Project>(conn)?;
    
      let project_14 = projects::table
        .filter(projects::columns::name.eq("DevOps Automation"))
        .first::<Project>(conn)?;
    
      let project_15 = projects::table
        .filter(projects::columns::name.eq("Computer Vision Project"))
        .first::<Project>(conn)?;
    
      let project_16 = projects::table
        .filter(projects::columns::name.eq("GraphQL API"))
        .first::<Project>(conn)?;
    
      let project_17 = projects::table
        .filter(projects::columns::name.eq("Serverless Function Development"))
        .first::<Project>(conn)?;
    
      let project_18 = projects::table
        .filter(projects::columns::name.eq("Real-time Communication"))
        .first::<Project>(conn)?;
    
      let project_19 = projects::table
        .filter(projects::columns::name.eq("Big Data Processing (Spark)"))
        .first::<Project>(conn)?;
    
      let project_20 = projects::table
        .filter(projects::columns::name.eq("Robotics Software Development"))
        .first::<Project>(conn)?;

    // --- Insert Images ---
    let new_profile_image_data = vec![
        NewUserImage::new("user_1_profile.jpg", user_1.user_id),
        NewUserImage::new("user_2_avatar.png", user_2.user_id),
        NewUserImage::new("user_3_photo.gif", user_3.user_id),
        NewUserImage::new("user_4_image.jpeg", user_4.user_id),
        NewUserImage::new("user_5_profile_pic.webp", user_5.user_id),
        NewUserImage::new("user_6_avatar_small.png", user_6.user_id),
        NewUserImage::new("user_7_headshot.jpg", user_7.user_id),
        NewUserImage::new("user_8_profile_image.jpeg", user_8.user_id),
        NewUserImage::new("user_9_display_pic.gif", user_9.user_id),
        NewUserImage::new("user_10_profile_01.png", user_10.user_id),
        NewUserImage::new("user_11_avatar_v2.jpg", user_11.user_id),
        NewUserImage::new("user_12_image_001.webp", user_12.user_id),
      ];

      let new_project_image_data = vec![
        NewProjectImage::new("project_1_screenshot.png", user_1.user_id, project_1.project_id),
        NewProjectImage::new("project_2_logo.svg", user_2.user_id, project_2.project_id),
        NewProjectImage::new("project_3_preview.jpg", user_3.user_id, project_3.project_id),
        NewProjectImage::new("project_4_icon.png", user_4.user_id, project_4.project_id),
        NewProjectImage::new("project_5_banner.jpeg", user_5.user_id, project_5.project_id),
        NewProjectImage::new("project_6_ui_design.webp", user_6.user_id, project_6.project_id),
        NewProjectImage::new("project_7_flowchart.gif", user_7.user_id, project_7.project_id),
        NewProjectImage::new("project_8_diagram.png", user_8.user_id, project_8.project_id),
      ];

    diesel::insert_into(images::table)
        .values(&new_profile_image_data)
        .execute(conn)?;

    diesel::insert_into(images::table)
        .values(&new_project_image_data)
        .execute(conn)?;

    // --- Insert Favourites ---
    let new_favourites_data = vec![
        NewFavourite::new(user_1.user_id, project_5.project_id),
        NewFavourite::new(user_3.user_id, project_12.project_id),
        NewFavourite::new(user_7.user_id, project_2.project_id),
        NewFavourite::new(user_15.user_id, project_18.project_id),
        NewFavourite::new(user_2.user_id, project_9.project_id),
        NewFavourite::new(user_18.user_id, project_4.project_id),
        NewFavourite::new(user_10.user_id, project_15.project_id),
        NewFavourite::new(user_5.user_id, project_1.project_id),
        NewFavourite::new(user_12.user_id, project_11.project_id),
        NewFavourite::new(user_19.user_id, project_7.project_id),
        NewFavourite::new(user_8.user_id, project_19.project_id),
        NewFavourite::new(user_16.user_id, project_3.project_id),
        NewFavourite::new(user_4.user_id, project_17.project_id),
        NewFavourite::new(user_11.user_id, project_8.project_id),
        NewFavourite::new(user_20.user_id, project_14.project_id),
        NewFavourite::new(user_6.user_id, project_6.project_id),
        NewFavourite::new(user_14.user_id, project_20.project_id),
        NewFavourite::new(user_9.user_id, project_10.project_id),
        NewFavourite::new(user_17.user_id, project_16.project_id),
        NewFavourite::new(user_13.user_id, project_13.project_id),
        NewFavourite::new(user_1.user_id, project_18.project_id),
        NewFavourite::new(user_5.user_id, project_2.project_id),
        NewFavourite::new(user_10.user_id, project_7.project_id),
        NewFavourite::new(user_15.user_id, project_12.project_id),
        NewFavourite::new(user_20.user_id, project_1.project_id),
        NewFavourite::new(user_3.user_id, project_19.project_id),
        NewFavourite::new(user_8.user_id, project_6.project_id),
        NewFavourite::new(user_12.user_id, project_16.project_id),
        NewFavourite::new(user_17.user_id, project_4.project_id),
        NewFavourite::new(user_6.user_id, project_11.project_id),
      ];

    diesel::insert_into(favourites::table)
        .values(&new_favourites_data)
        .execute(conn)?;

    // --- Insert Follows ---
    let new_follows_data = vec![
        NewFollow::new(user_1.user_id, user_2.user_id),
        NewFollow::new(user_3.user_id, user_7.user_id),
        NewFollow::new(user_5.user_id, user_12.user_id),
        NewFollow::new(user_9.user_id, user_18.user_id),
        NewFollow::new(user_15.user_id, user_4.user_id),
        NewFollow::new(user_2.user_id, user_8.user_id),
        NewFollow::new(user_7.user_id, user_14.user_id),
        NewFollow::new(user_11.user_id, user_1.user_id),
        NewFollow::new(user_18.user_id, user_6.user_id),
        NewFollow::new(user_4.user_id, user_19.user_id),
        NewFollow::new(user_10.user_id, user_3.user_id),
        NewFollow::new(user_16.user_id, user_9.user_id),
        NewFollow::new(user_6.user_id, user_13.user_id),
        NewFollow::new(user_13.user_id, user_20.user_id),
        NewFollow::new(user_20.user_id, user_5.user_id),
        NewFollow::new(user_8.user_id, user_11.user_id),
        NewFollow::new(user_17.user_id, user_16.user_id),
        NewFollow::new(user_1.user_id, user_10.user_id),
        NewFollow::new(user_12.user_id, user_17.user_id),
        NewFollow::new(user_19.user_id, user_15.user_id),
        NewFollow::new(user_2.user_id, user_5.user_id),
        NewFollow::new(user_4.user_id, user_9.user_id),
        NewFollow::new(user_6.user_id, user_11.user_id),
        NewFollow::new(user_8.user_id, user_13.user_id),
        NewFollow::new(user_10.user_id, user_15.user_id),
        NewFollow::new(user_12.user_id, user_17.user_id),
        NewFollow::new(user_14.user_id, user_19.user_id),
        NewFollow::new(user_16.user_id, user_1.user_id),
        NewFollow::new(user_18.user_id, user_3.user_id),
        NewFollow::new(user_20.user_id, user_7.user_id),
        NewFollow::new(user_3.user_id, user_1.user_id),
        NewFollow::new(user_7.user_id, user_5.user_id),
        NewFollow::new(user_11.user_id, user_9.user_id),
        NewFollow::new(user_15.user_id, user_13.user_id),
        NewFollow::new(user_19.user_id, user_17.user_id),
        NewFollow::new(user_1.user_id, user_20.user_id),
        NewFollow::new(user_5.user_id, user_18.user_id),
        NewFollow::new(user_9.user_id, user_16.user_id),
        NewFollow::new(user_13.user_id, user_14.user_id),
        NewFollow::new(user_17.user_id, user_12.user_id),
      ];

    diesel::insert_into(follows::table)
        .values(&new_follows_data)
        .execute(conn)?;

    // --- Insert Comments ---
    let new_comments_data = vec![
        NewComment::new("Interesting concept!", user_3.user_id, project_7.project_id),
        NewComment::new("How did you achieve this?", user_15.user_id, project_2.project_id),
        NewComment::new("Needs more testing.", user_8.user_id, project_14.project_id),
        NewComment::new("I love the UI!", user_2.user_id, project_9.project_id),
        NewComment::new("Any plans for mobile?", user_11.user_id, project_4.project_id),
        NewComment::new("Great job on the documentation.", user_18.user_id, project_17.project_id),
        NewComment::new("This is really innovative.", user_5.user_id, project_1.project_id),
        NewComment::new("Could you explain this part?", user_19.user_id, project_12.project_id),
        NewComment::new("Keep up the good work!", user_1.user_id, project_6.project_id),
        NewComment::new("I'm excited to see the next steps.", user_10.user_id, project_19.project_id),
        NewComment::new("This looks promising.", user_14.user_id, project_3.project_id),
        NewComment::new("Have you considered using X?", user_7.user_id, project_16.project_id),
        NewComment::new("Awesome features!", user_20.user_id, project_10.project_id),
        NewComment::new("I'd like to contribute.", user_4.user_id, project_13.project_id),
        NewComment::new("This solved my problem!", user_12.user_id, project_8.project_id),
        NewComment::new("Is there an API?", user_17.user_id, project_15.project_id),
        NewComment::new("The design is clean.", user_6.user_id, project_20.project_id),
        NewComment::new("How scalable is this?", user_9.user_id, project_5.project_id),
        NewComment::new("Any tutorials available?", user_13.user_id, project_11.project_id),
        NewComment::new("Fantastic!", user_16.user_id, project_18.project_id),
        NewComment::new("This is a game changer.", user_2.user_id, project_3.project_id),
        NewComment::new("I'm testing this out now.", user_11.user_id, project_17.project_id),
        NewComment::new("The performance is impressive.", user_18.user_id, project_8.project_id),
        NewComment::new("Great for beginners!", user_5.user_id, project_15.project_id),
        NewComment::new("Any known limitations?", user_19.user_id, project_1.project_id),
        NewComment::new("Love the color scheme.", user_1.user_id, project_10.project_id),
        NewComment::new("This could be really useful.", user_10.user_id, project_20.project_id),
        NewComment::new("What are the dependencies?", user_14.user_id, project_6.project_id),
        NewComment::new("Solid work!", user_7.user_id, project_12.project_id),
        NewComment::new("Looking forward to updates.", user_20.user_id, project_4.project_id),
        NewComment::new("Easy to use!", user_4.user_id, project_19.project_id),
        NewComment::new("Clear and concise.", user_12.user_id, project_7.project_id),
        NewComment::new("Well done!", user_17.user_id, project_2.project_id),
        NewComment::new("Simple and effective.", user_6.user_id, project_14.project_id),
        NewComment::new("I have a question about...", user_9.user_id, project_9.project_id),
        NewComment::new("This is well-structured.", user_13.user_id, project_5.project_id),
        NewComment::new("Excellent implementation!", user_16.user_id, project_11.project_id),
        NewComment::new("Very helpful.", user_3.user_id, project_18.project_id),
        NewComment::new("Thank you for sharing!", user_15.user_id, project_1.project_id),
        NewComment::new("Nice!", user_8.user_id, project_10.project_id),
        NewComment::new("I'm trying to integrate this.", user_2.user_id, project_20.project_id),
        NewComment::new("Any plans for integration with Y?", user_11.user_id, project_6.project_id),
        NewComment::new("This is exactly what I needed.", user_18.user_id, project_12.project_id),
        NewComment::new("Good job on the responsiveness.", user_5.user_id, project_4.project_id),
        NewComment::new("How can I contribute?", user_19.user_id, project_19.project_id),
        NewComment::new("The documentation is great.", user_1.user_id, project_7.project_id),
        NewComment::new("I'm having trouble with...", user_10.user_id, project_15.project_id),
        NewComment::new("This is a solid foundation.", user_14.user_id, project_3.project_id),
        NewComment::new("Impressive work!", user_7.user_id, project_16.project_id),
        NewComment::new("I'll definitely be using this.", user_20.user_id, project_8.project_id),
      ];

    diesel::insert_into(comments::table)
        .values(&new_comments_data)
        .execute(conn)?;

        let comment_1 = comments::table
        .filter(comments::columns::text.eq("Interesting concept!"))
        .first::<Comment>(conn)?;
    
      let comment_2 = comments::table
        .filter(comments::columns::text.eq("How did you achieve this?"))
        .first::<Comment>(conn)?;
    
      let comment_3 = comments::table
        .filter(comments::columns::text.eq("Needs more testing."))
        .first::<Comment>(conn)?;
    
      let comment_4 = comments::table
        .filter(comments::columns::text.eq("I love the UI!"))
        .first::<Comment>(conn)?;
    
      let comment_5 = comments::table
        .filter(comments::columns::text.eq("Any plans for mobile?"))
        .first::<Comment>(conn)?;
    
      let comment_6 = comments::table
        .filter(comments::columns::text.eq("Great job on the documentation."))
        .first::<Comment>(conn)?;
    
      let comment_7 = comments::table
        .filter(comments::columns::text.eq("This is really innovative."))
        .first::<Comment>(conn)?;
    
      let comment_8 = comments::table
        .filter(comments::columns::text.eq("Could you explain this part?"))
        .first::<Comment>(conn)?;
    
      let comment_9 = comments::table
        .filter(comments::columns::text.eq("Keep up the good work!"))
        .first::<Comment>(conn)?;
    
      let comment_10 = comments::table
        .filter(comments::columns::text.eq("I'm excited to see the next steps."))
        .first::<Comment>(conn)?;
    
      let comment_11 = comments::table
        .filter(comments::columns::text.eq("This looks promising."))
        .first::<Comment>(conn)?;
    
      let comment_12 = comments::table
        .filter(comments::columns::text.eq("Have you considered using X?"))
        .first::<Comment>(conn)?;
    
      let comment_13 = comments::table
        .filter(comments::columns::text.eq("Awesome features!"))
        .first::<Comment>(conn)?;
    
      let comment_14 = comments::table
        .filter(comments::columns::text.eq("I'd like to contribute."))
        .first::<Comment>(conn)?;
    
      let comment_15 = comments::table
        .filter(comments::columns::text.eq("This solved my problem!"))
        .first::<Comment>(conn)?;
    
      let comment_16 = comments::table
        .filter(comments::columns::text.eq("Is there an API?"))
        .first::<Comment>(conn)?;
    
      let comment_17 = comments::table
        .filter(comments::columns::text.eq("The design is clean."))
        .first::<Comment>(conn)?;
    
      let comment_18 = comments::table
        .filter(comments::columns::text.eq("How scalable is this?"))
        .first::<Comment>(conn)?;
    
      let comment_19 = comments::table
        .filter(comments::columns::text.eq("Any tutorials available?"))
        .first::<Comment>(conn)?;
    
      let comment_20 = comments::table
        .filter(comments::columns::text.eq("Fantastic!"))
        .first::<Comment>(conn)?;
    
      let comment_21 = comments::table
        .filter(comments::columns::text.eq("This is a game changer."))
        .first::<Comment>(conn)?;
    
      let comment_22 = comments::table
        .filter(comments::columns::text.eq("I'm testing this out now."))
        .first::<Comment>(conn)?;
    
      let comment_23 = comments::table
        .filter(comments::columns::text.eq("The performance is impressive."))
        .first::<Comment>(conn)?;
    
      let comment_24 = comments::table
        .filter(comments::columns::text.eq("Great for beginners!"))
        .first::<Comment>(conn)?;
    
      let comment_25 = comments::table
        .filter(comments::columns::text.eq("Any known limitations?"))
        .first::<Comment>(conn)?;

    // --- Insert Replies ---
    let new_replies_data = vec![
        NewReply::new("You're welcome!", user_5.user_id),
        NewReply::new("Glad it helped!", user_15.user_id),
        NewReply::new("Working on it!", user_8.user_id),
        NewReply::new("Thank you!", user_2.user_id),
        NewReply::new("Yes, in the future.", user_11.user_id),
        NewReply::new("Appreciate that!", user_18.user_id),
        NewReply::new("Thanks for the feedback!", user_5.user_id),
        NewReply::new("Let me clarify...", user_19.user_id),
        NewReply::new("Glad you like it!", user_1.user_id),
        NewReply::new("Stay tuned!", user_10.user_id),
        NewReply::new("Working on improvements.", user_14.user_id),
        NewReply::new("That's an interesting idea!", user_7.user_id),
        NewReply::new("Glad to hear!", user_20.user_id),
        NewReply::new("We'd love your contributions!", user_4.user_id),
        NewReply::new("Awesome!", user_12.user_id),
        NewReply::new("Yes, check the API docs.", user_17.user_id),
        NewReply::new("Thanks!", user_6.user_id),
        NewReply::new("It's designed to be scalable.", user_9.user_id),
        NewReply::new("Yes, there are some on our website.", user_13.user_id),
        NewReply::new("Thanks!", user_16.user_id),
        NewReply::new("Glad you think so!", user_2.user_id),
        NewReply::new("Let me know if you have questions.", user_11.user_id),
        NewReply::new("We appreciate that!", user_18.user_id),
        NewReply::new("Great for all levels!", user_5.user_id),
        NewReply::new("See the FAQ for details.", user_19.user_id),
        NewReply::new("We aimed for that!", user_1.user_id),
        NewReply::new("It can be very useful indeed.", user_10.user_id),
        NewReply::new("Those are listed in the README.", user_14.user_id),
        NewReply::new("Glad you think so!", user_7.user_id),
        NewReply::new("Exciting times ahead!", user_20.user_id),
      ];

    diesel::insert_into(replies::table)
        .values(&new_replies_data)
        .execute(conn)?;

        let reply_1 = replies::table
        .order(replies::columns::reply_id.desc())
        .first::<Reply>(conn)?;
    
      let reply_2 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(1)
        .first::<Reply>(conn)?;
    
      let reply_3 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(2)
        .first::<Reply>(conn)?;
    
      let reply_4 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(3)
        .first::<Reply>(conn)?;
    
      let reply_5 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(4)
        .first::<Reply>(conn)?;
    
      let reply_6 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(5)
        .first::<Reply>(conn)?;
    
      let reply_7 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(6)
        .first::<Reply>(conn)?;
    
      let reply_8 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(7)
        .first::<Reply>(conn)?;
    
      let reply_9 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(8)
        .first::<Reply>(conn)?;
    
      let reply_10 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(9)
        .first::<Reply>(conn)?;
    
      let reply_11 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(10)
        .first::<Reply>(conn)?;
    
      let reply_12 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(11)
        .first::<Reply>(conn)?;
    
      let reply_13 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(12)
        .first::<Reply>(conn)?;
    
      let reply_14 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(13)
        .first::<Reply>(conn)?;
    
      let reply_15 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(14)
        .first::<Reply>(conn)?;
    
      let reply_16 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(15)
        .first::<Reply>(conn)?;
    
      let reply_17 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(16)
        .first::<Reply>(conn)?;
    
      let reply_18 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(17)
        .first::<Reply>(conn)?;
    
      let reply_19 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(18)
        .first::<Reply>(conn)?;
    
      let reply_20 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(19)
        .first::<Reply>(conn)?;
    
      let reply_21 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(20)
        .first::<Reply>(conn)?;
    
      let reply_22 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(21)
        .first::<Reply>(conn)?;
    
      let reply_23 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(22)
        .first::<Reply>(conn)?;
    
      let reply_24 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(23)
        .first::<Reply>(conn)?;
    
      let reply_25 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(24)
        .first::<Reply>(conn)?;
    
      let reply_26 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(25)
        .first::<Reply>(conn)?;
    
      let reply_27 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(26)
        .first::<Reply>(conn)?;
    
      let reply_28 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(27)
        .first::<Reply>(conn)?;
    
      let reply_29 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(28)
        .first::<Reply>(conn)?;
    
      let reply_30 = replies::table
        .order(replies::columns::reply_id.desc())
        .offset(29)
        .first::<Reply>(conn)?;

    // --- Insert Threads ---
    let new_threads_data = vec![
        NewThread::new(comment_1.comment_id, reply_1.reply_id),
        NewThread::new(comment_2.comment_id, reply_2.reply_id),
        NewThread::new(comment_3.comment_id, reply_3.reply_id),
        NewThread::new(comment_4.comment_id, reply_4.reply_id),
        NewThread::new(comment_5.comment_id, reply_5.reply_id),
        NewThread::new(comment_6.comment_id, reply_6.reply_id),
        NewThread::new(comment_7.comment_id, reply_7.reply_id),
        NewThread::new(comment_8.comment_id, reply_8.reply_id),
        NewThread::new(comment_9.comment_id, reply_9.reply_id),
        NewThread::new(comment_10.comment_id, reply_10.reply_id),
        NewThread::new(comment_11.comment_id, reply_11.reply_id),
        NewThread::new(comment_12.comment_id, reply_12.reply_id),
        NewThread::new(comment_13.comment_id, reply_13.reply_id),
        NewThread::new(comment_14.comment_id, reply_14.reply_id),
        NewThread::new(comment_15.comment_id, reply_15.reply_id),
        NewThread::new(comment_16.comment_id, reply_16.reply_id),
        NewThread::new(comment_17.comment_id, reply_17.reply_id),
        NewThread::new(comment_18.comment_id, reply_18.reply_id),
        NewThread::new(comment_19.comment_id, reply_19.reply_id),
        NewThread::new(comment_20.comment_id, reply_20.reply_id),
        NewThread::new(comment_21.comment_id, reply_21.reply_id),
        NewThread::new(comment_22.comment_id, reply_22.reply_id),
        NewThread::new(comment_23.comment_id, reply_23.reply_id),
        NewThread::new(comment_24.comment_id, reply_24.reply_id),
        NewThread::new(comment_25.comment_id, reply_25.reply_id),
        NewThread::new(comment_1.comment_id, reply_26.reply_id),
        NewThread::new(comment_5.comment_id, reply_27.reply_id),
        NewThread::new(comment_10.comment_id, reply_28.reply_id),
        NewThread::new(comment_15.comment_id, reply_29.reply_id),
        NewThread::new(comment_20.comment_id, reply_30.reply_id),
      ];

    diesel::insert_into(threads::table)
        .values(&new_threads_data)
        .execute(conn)?;

    // --- Insert Likes ---
    let new_likes_data = vec![
      NewLike { user_id: user_1.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_3.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_5.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_7.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_9.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_11.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_13.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_15.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_17.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_19.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_2.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_4.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_6.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_8.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_10.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_12.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_14.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_16.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_18.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_20.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_1.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_5.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_9.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_13.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_17.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_2.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_6.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_10.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_14.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_18.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_3.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_7.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_11.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_15.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_19.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_4.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_8.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_12.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_16.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_20.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_2.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_7.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_12.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_17.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_4.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_9.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_14.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_19.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_6.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_11.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_16.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_1.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_8.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_15.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_20.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_3.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_10.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_17.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_5.user_id, date_liked: Local::now().date_naive() },
      NewLike { user_id: user_12.user_id, date_liked: Local::now().date_naive() },
    ];

    diesel::insert_into(likes::table)
        .values(&new_likes_data)
        .execute(conn)?;

        let like_1 = likes::table
        .filter(likes::columns::user_id.eq(user_1.user_id))
        .first::<Like>(conn)?;
    
      let like_2 = likes::table
        .filter(likes::columns::user_id.eq(user_2.user_id))
        .first::<Like>(conn)?;
    
      let like_3 = likes::table
        .filter(likes::columns::user_id.eq(user_3.user_id))
        .first::<Like>(conn)?;
    
      let like_4 = likes::table
        .filter(likes::columns::user_id.eq(user_4.user_id))
        .first::<Like>(conn)?;
    
      let like_5 = likes::table
        .filter(likes::columns::user_id.eq(user_5.user_id))
        .first::<Like>(conn)?;
    
      let like_6 = likes::table
        .filter(likes::columns::user_id.eq(user_6.user_id))
        .first::<Like>(conn)?;
    
      let like_7 = likes::table
        .filter(likes::columns::user_id.eq(user_7.user_id))
        .first::<Like>(conn)?;
    
      let like_8 = likes::table
        .filter(likes::columns::user_id.eq(user_8.user_id))
        .first::<Like>(conn)?;
    
      let like_9 = likes::table
        .filter(likes::columns::user_id.eq(user_9.user_id))
        .first::<Like>(conn)?;
    
      let like_10 = likes::table
        .filter(likes::columns::user_id.eq(user_10.user_id))
        .first::<Like>(conn)?;
    
      let like_11 = likes::table
        .filter(likes::columns::user_id.eq(user_11.user_id))
        .first::<Like>(conn)?;
    
      let like_12 = likes::table
        .filter(likes::columns::user_id.eq(user_12.user_id))
        .first::<Like>(conn)?;
    
      let like_13 = likes::table
        .filter(likes::columns::user_id.eq(user_13.user_id))
        .first::<Like>(conn)?;
    
      let like_14 = likes::table
        .filter(likes::columns::user_id.eq(user_14.user_id))
        .first::<Like>(conn)?;
    
      let like_15 = likes::table
        .filter(likes::columns::user_id.eq(user_15.user_id))
        .first::<Like>(conn)?;
    
      let like_16 = likes::table
        .filter(likes::columns::user_id.eq(user_16.user_id))
        .first::<Like>(conn)?;
    
      let like_17 = likes::table
        .filter(likes::columns::user_id.eq(user_17.user_id))
        .first::<Like>(conn)?;
    
      let like_18 = likes::table
        .filter(likes::columns::user_id.eq(user_18.user_id))
        .first::<Like>(conn)?;
    
      let like_19 = likes::table
        .filter(likes::columns::user_id.eq(user_19.user_id))
        .first::<Like>(conn)?;
    
      let like_20 = likes::table
        .filter(likes::columns::user_id.eq(user_20.user_id))
        .first::<Like>(conn)?;
    
      let like_21 = likes::table
        .filter(likes::columns::user_id.eq(user_1.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_22 = likes::table
        .filter(likes::columns::user_id.eq(user_5.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_23 = likes::table
        .filter(likes::columns::user_id.eq(user_9.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_24 = likes::table
        .filter(likes::columns::user_id.eq(user_13.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_25 = likes::table
        .filter(likes::columns::user_id.eq(user_17.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_26 = likes::table
        .filter(likes::columns::user_id.eq(user_2.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_27 = likes::table
        .filter(likes::columns::user_id.eq(user_6.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_28 = likes::table
        .filter(likes::columns::user_id.eq(user_10.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_29 = likes::table
        .filter(likes::columns::user_id.eq(user_14.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_30 = likes::table
        .filter(likes::columns::user_id.eq(user_18.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_31 = likes::table
        .filter(likes::columns::user_id.eq(user_3.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_32 = likes::table
        .filter(likes::columns::user_id.eq(user_7.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_33 = likes::table
        .filter(likes::columns::user_id.eq(user_11.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_34 = likes::table
        .filter(likes::columns::user_id.eq(user_15.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_35 = likes::table
        .filter(likes::columns::user_id.eq(user_19.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_36 = likes::table
        .filter(likes::columns::user_id.eq(user_4.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_37 = likes::table
        .filter(likes::columns::user_id.eq(user_8.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_38 = likes::table
        .filter(likes::columns::user_id.eq(user_12.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_39 = likes::table
        .filter(likes::columns::user_id.eq(user_16.user_id))
        .offset(1)
        .first::<Like>(conn)?;
    
      let like_40 = likes::table
        .filter(likes::columns::user_id.eq(user_20.user_id))
        .offset(1)
        .first::<Like>(conn)?;

    // --- Insert Dislikes ---
    let new_dislikes_data = vec![
      NewDislike { user_id: user_2.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_5.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_8.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_11.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_14.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_17.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_20.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_1.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_4.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_7.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_10.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_13.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_16.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_19.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_3.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_6.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_9.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_12.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_15.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_18.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_1.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_6.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_11.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_16.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_3.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_8.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_13.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_18.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_5.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_10.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_15.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_20.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_2.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_7.user_id, date_disliked: Local::now().date_naive() },
      NewDislike { user_id: user_12.user_id, date_disliked: Local::now().date_naive() },
    ];

    diesel::insert_into(dislikes::table)
        .values(&new_dislikes_data)
        .execute(conn)?;

        let dislike_1 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .first::<Dislike>(conn)?;
    
      let dislike_2 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(1)
        .first::<Dislike>(conn)?;
    
      let dislike_3 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(2)
        .first::<Dislike>(conn)?;
    
      let dislike_4 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(3)
        .first::<Dislike>(conn)?;
    
      let dislike_5 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(4)
        .first::<Dislike>(conn)?;
    
      let dislike_6 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(5)
        .first::<Dislike>(conn)?;
    
      let dislike_7 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(6)
        .first::<Dislike>(conn)?;
    
      let dislike_8 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(7)
        .first::<Dislike>(conn)?;
    
      let dislike_9 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(8)
        .first::<Dislike>(conn)?;
    
      let dislike_10 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(9)
        .first::<Dislike>(conn)?;
    
      let dislike_11 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(10)
        .first::<Dislike>(conn)?;
    
      let dislike_12 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(11)
        .first::<Dislike>(conn)?;
    
      let dislike_13 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(12)
        .first::<Dislike>(conn)?;
    
      let dislike_14 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(13)
        .first::<Dislike>(conn)?;
    
      let dislike_15 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(14)
        .first::<Dislike>(conn)?;
    
      let dislike_16 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(15)
        .first::<Dislike>(conn)?;
    
      let dislike_17 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(16)
        .first::<Dislike>(conn)?;
    
      let dislike_18 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(17)
        .first::<Dislike>(conn)?;
    
      let dislike_19 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(18)
        .first::<Dislike>(conn)?;
    
      let dislike_20 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(19)
        .first::<Dislike>(conn)?;
    
      let dislike_21 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(20)
        .first::<Dislike>(conn)?;
    
      let dislike_22 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(21)
        .first::<Dislike>(conn)?;
    
      let dislike_23 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(22)
        .first::<Dislike>(conn)?;
    
      let dislike_24 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(23)
        .first::<Dislike>(conn)?;
    
      let dislike_25 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(24)
        .first::<Dislike>(conn)?;
    
      let dislike_26 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(25)
        .first::<Dislike>(conn)?;
    
      let dislike_27 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(26)
        .first::<Dislike>(conn)?;
    
      let dislike_28 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(27)
        .first::<Dislike>(conn)?;
    
      let dislike_29 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(28)
        .first::<Dislike>(conn)?;
    
      let dislike_30 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(29)
        .first::<Dislike>(conn)?;
    
      let dislike_31 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(30)
        .first::<Dislike>(conn)?;
    
      let dislike_32 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(31)
        .first::<Dislike>(conn)?;
    
      let dislike_33 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(32)
        .first::<Dislike>(conn)?;
    
      let dislike_34 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(33)
        .first::<Dislike>(conn)?;
    
      let dislike_35 = dislikes::table
        .order(dislikes::columns::dislike_id.desc())
        .offset(34)
        .first::<Dislike>(conn)?;

    // --- Insert Comment Likes ---
    let new_comment_likes_data = vec![
        NewCommentLike { comment_id: comment_1.comment_id, like_id: like_1.like_id },
        NewCommentLike { comment_id: comment_2.comment_id, like_id: like_2.like_id },
        NewCommentLike { comment_id: comment_3.comment_id, like_id: like_3.like_id },
        NewCommentLike { comment_id: comment_4.comment_id, like_id: like_4.like_id },
        NewCommentLike { comment_id: comment_5.comment_id, like_id: like_5.like_id },
        NewCommentLike { comment_id: comment_6.comment_id, like_id: like_6.like_id },
        NewCommentLike { comment_id: comment_7.comment_id, like_id: like_7.like_id },
        NewCommentLike { comment_id: comment_8.comment_id, like_id: like_8.like_id },
        NewCommentLike { comment_id: comment_9.comment_id, like_id: like_9.like_id },
        NewCommentLike { comment_id: comment_10.comment_id, like_id: like_10.like_id },
        NewCommentLike { comment_id: comment_11.comment_id, like_id: like_11.like_id },
        NewCommentLike { comment_id: comment_12.comment_id, like_id: like_12.like_id },
        NewCommentLike { comment_id: comment_13.comment_id, like_id: like_13.like_id },
        NewCommentLike { comment_id: comment_14.comment_id, like_id: like_14.like_id },
        NewCommentLike { comment_id: comment_15.comment_id, like_id: like_15.like_id },
        NewCommentLike { comment_id: comment_16.comment_id, like_id: like_16.like_id },
        NewCommentLike { comment_id: comment_17.comment_id, like_id: like_17.like_id },
        NewCommentLike { comment_id: comment_18.comment_id, like_id: like_18.like_id },
        NewCommentLike { comment_id: comment_19.comment_id, like_id: like_19.like_id },
        NewCommentLike { comment_id: comment_20.comment_id, like_id: like_20.like_id },
        NewCommentLike { comment_id: comment_21.comment_id, like_id: like_21.like_id },
        NewCommentLike { comment_id: comment_22.comment_id, like_id: like_22.like_id },
        NewCommentLike { comment_id: comment_23.comment_id, like_id: like_23.like_id },
        NewCommentLike { comment_id: comment_24.comment_id, like_id: like_24.like_id },
        NewCommentLike { comment_id: comment_25.comment_id, like_id: like_25.like_id },
        NewCommentLike { comment_id: comment_1.comment_id, like_id: like_26.like_id },
        NewCommentLike { comment_id: comment_3.comment_id, like_id: like_27.like_id },
        NewCommentLike { comment_id: comment_5.comment_id, like_id: like_28.like_id },
        NewCommentLike { comment_id: comment_7.comment_id, like_id: like_29.like_id },
        NewCommentLike { comment_id: comment_9.comment_id, like_id: like_30.like_id },
      ];

    diesel::insert_into(comment_likes::table)
        .values(&new_comment_likes_data)
        .execute(conn)?;

    // --- Insert Comment Dislikes ---
    let new_comment_dislikes_data = vec![
        NewCommentDislike { comment_id: comment_1.comment_id, dislike_id: dislike_1.dislike_id },
        NewCommentDislike { comment_id: comment_2.comment_id, dislike_id: dislike_2.dislike_id },
        NewCommentDislike { comment_id: comment_3.comment_id, dislike_id: dislike_3.dislike_id },
        NewCommentDislike { comment_id: comment_4.comment_id, dislike_id: dislike_4.dislike_id },
        NewCommentDislike { comment_id: comment_5.comment_id, dislike_id: dislike_5.dislike_id },
        NewCommentDislike { comment_id: comment_6.comment_id, dislike_id: dislike_6.dislike_id },
        NewCommentDislike { comment_id: comment_7.comment_id, dislike_id: dislike_7.dislike_id },
        NewCommentDislike { comment_id: comment_8.comment_id, dislike_id: dislike_8.dislike_id },
        NewCommentDislike { comment_id: comment_9.comment_id, dislike_id: dislike_9.dislike_id },
        NewCommentDislike { comment_id: comment_10.comment_id, dislike_id: dislike_10.dislike_id },
        NewCommentDislike { comment_id: comment_11.comment_id, dislike_id: dislike_11.dislike_id },
        NewCommentDislike { comment_id: comment_12.comment_id, dislike_id: dislike_12.dislike_id },
        NewCommentDislike { comment_id: comment_13.comment_id, dislike_id: dislike_13.dislike_id },
        NewCommentDislike { comment_id: comment_14.comment_id, dislike_id: dislike_14.dislike_id },
        NewCommentDislike { comment_id: comment_15.comment_id, dislike_id: dislike_15.dislike_id },
        NewCommentDislike { comment_id: comment_1.comment_id, dislike_id: dislike_16.dislike_id },
        NewCommentDislike { comment_id: comment_3.comment_id, dislike_id: dislike_17.dislike_id },
        NewCommentDislike { comment_id: comment_5.comment_id, dislike_id: dislike_18.dislike_id },
        NewCommentDislike { comment_id: comment_7.comment_id, dislike_id: dislike_19.dislike_id },
        NewCommentDislike { comment_id: comment_9.comment_id, dislike_id: dislike_20.dislike_id },
      ];

    diesel::insert_into(comment_dislikes::table)
        .values(&new_comment_dislikes_data)
        .execute(conn)?;

    // --- Insert Reply Likes ---
    let new_reply_likes_data = vec![
        NewReplyLike { reply_id: reply_1.reply_id, like_id: like_31.like_id },
        NewReplyLike { reply_id: reply_2.reply_id, like_id: like_32.like_id },
        NewReplyLike { reply_id: reply_3.reply_id, like_id: like_33.like_id },
        NewReplyLike { reply_id: reply_4.reply_id, like_id: like_34.like_id },
        NewReplyLike { reply_id: reply_5.reply_id, like_id: like_35.like_id },
        NewReplyLike { reply_id: reply_6.reply_id, like_id: like_36.like_id },
        NewReplyLike { reply_id: reply_7.reply_id, like_id: like_37.like_id },
        NewReplyLike { reply_id: reply_8.reply_id, like_id: like_38.like_id },
        NewReplyLike { reply_id: reply_9.reply_id, like_id: like_39.like_id },
        NewReplyLike { reply_id: reply_10.reply_id, like_id: like_40.like_id },
      ];

    diesel::insert_into(reply_likes::table)
        .values(&new_reply_likes_data)
        .execute(conn)?;

    // --- Insert Reply Dislikes ---
    let new_reply_dislikes_data = vec![
        NewReplyDislike { reply_id: reply_1.reply_id, dislike_id: dislike_21.dislike_id },
        NewReplyDislike { reply_id: reply_2.reply_id, dislike_id: dislike_22.dislike_id },
        NewReplyDislike { reply_id: reply_3.reply_id, dislike_id: dislike_23.dislike_id },
        NewReplyDislike { reply_id: reply_4.reply_id, dislike_id: dislike_24.dislike_id },
        NewReplyDislike { reply_id: reply_5.reply_id, dislike_id: dislike_25.dislike_id },
        NewReplyDislike { reply_id: reply_6.reply_id, dislike_id: dislike_26.dislike_id },
        NewReplyDislike { reply_id: reply_7.reply_id, dislike_id: dislike_27.dislike_id },
        NewReplyDislike { reply_id: reply_8.reply_id, dislike_id: dislike_28.dislike_id },
        NewReplyDislike { reply_id: reply_9.reply_id, dislike_id: dislike_29.dislike_id },
        NewReplyDislike { reply_id: reply_10.reply_id, dislike_id: dislike_30.dislike_id },
        NewReplyDislike { reply_id: reply_11.reply_id, dislike_id: dislike_31.dislike_id },
        NewReplyDislike { reply_id: reply_12.reply_id, dislike_id: dislike_32.dislike_id },
        NewReplyDislike { reply_id: reply_13.reply_id, dislike_id: dislike_33.dislike_id },
        NewReplyDislike { reply_id: reply_14.reply_id, dislike_id: dislike_34.dislike_id },
        NewReplyDislike { reply_id: reply_15.reply_id, dislike_id: dislike_35.dislike_id },
      ];

    diesel::insert_into(reply_dislikes::table)
        .values(&new_reply_dislikes_data)
        .execute(conn)?;
    Ok(())
}
