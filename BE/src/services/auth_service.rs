use bcrypt::verify;
use crate::security::jwt::create_jwt;
use sqlx::{Pool, Postgres};
use crate::db::classes_repo::find_class_by_id;
use crate::entity::auth_user::AuthUser;
use crate::db::teacher_student_repo::{get_teacher_by_id, get_student_by_id};
use crate::entity::users::Role;
use crate::models::login_response::LoginResponse;
use anyhow::anyhow;

pub async fn login_user(db: &Pool<Postgres>, email: &str, password: &str, secret: &str) -> anyhow::Result<LoginResponse> {
    let record = sqlx::query_as::<_, AuthUser>(
        "SELECT id, user_id, email, password, role, created_at FROM auth_users WHERE email = $1"
    )
        .bind(email)
        .fetch_one(db)
        .await?;

    let is_valid = verify(password, &record.password)?;
    if !is_valid {
        return Err(anyhow::anyhow!("Invalid credentials"));
    }

    let role = Role::from(record.role.clone());

    let (image_url, first_name, last_name, class_id_option) = match role {
        Role::Teacher => {
            let teacher_opt = get_teacher_by_id(db, record.user_id).await?;
            let teacher = teacher_opt.ok_or_else(|| anyhow!("Teacher record not found user"))?;

            (
                teacher.user.image_url,
                teacher.user.first_name,
                teacher.user.last_name,
                teacher.homeroom_class_id
            )
        },

        Role::Student => {
            let student_opt = get_student_by_id(db, record.user_id).await?;
            let student = student_opt.ok_or_else(|| anyhow!("Student record not found user"))?;

            (
                student.user.image_url,
                student.user.first_name,
                student.user.last_name,
                Some(student.class_id),
            )
        },
        Role::Admin => {
            ("".to_string(), "Admin".to_string(), "".to_string(), None)
        }
    };

    let class_name = if let Some(id) = class_id_option {
        let id_str = id.to_string();

        find_class_by_id(db, &id_str)
            .await?
            .unwrap_or_else(|| "".to_string())
    } else {
        "".to_string()
    };

    let jwt_token = create_jwt(&record.user_id.to_string(), &record.role, &record.email, secret);
    let login_response = LoginResponse {
        uuid: record.user_id,
        access_token: jwt_token,
        refresh_token: "".to_string(),
        role,
        image_url,
        first_name,
        last_name,
        class: class_name,
    };
    Ok(login_response)
}