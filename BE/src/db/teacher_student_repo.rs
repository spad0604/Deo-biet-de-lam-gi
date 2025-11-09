use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::entity::user_entities::{TeacherEntity, StudentEntity};
use crate::entity::teacher::Teacher;
use crate::entity::student::Student;
use crate::entity::users::User;

pub async fn get_teacher_by_id(db: &Pool<Postgres>, id: Uuid) -> sqlx::Result<Option<Teacher>> {
    let entity = sqlx::query_as::<_, TeacherEntity>(
        "SELECT id, first_name, last_name, date_of_birth, phone_number, image_url, homeroom_class_id 
         FROM teachers WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(entity.map(|e| Teacher {
        id: e.id,
        user: User {
            first_name: e.first_name,
            last_name: e.last_name,
            date_of_birth: e.date_of_birth,
            phone_number: e.phone_number,
            image_url: e.image_url,
        },
        homeroom_class_id: e.homeroom_class_id,
    }))
}

pub async fn save_teacher_image_url(db: &Pool<Postgres>, image_url: &str, id: Uuid) -> sqlx::Result<()> {
    sqlx::query("UPDATE teachers SET image_url = $1 WHERE id = $2")
        .bind(image_url)
        .bind(id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn get_student_by_id(db: &Pool<Postgres>, id: Uuid) -> sqlx::Result<Option<Student>> {
    let entity = sqlx::query_as::<_, StudentEntity>(
        "SELECT id, first_name, last_name, date_of_birth, phone_number, image_url, class_id 
         FROM students WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(entity.map(|e| Student {
        id: e.id,
        user: User {
            first_name: e.first_name,
            last_name: e.last_name,
            date_of_birth: e.date_of_birth,
            phone_number: e.phone_number,
            image_url: e.image_url,
        },
        class_id: e.class_id,
    }))
}

pub async fn save_student_image_url(db: &Pool<Postgres>, image_url: &str, id: Uuid) -> sqlx::Result<()> {
    sqlx::query("UPDATE students SET image_url = $1 WHERE id = $2")
        .bind(image_url)
        .bind(id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn create_teacher(db: &Pool<Postgres>, teacher: TeacherEntity) -> sqlx::Result<TeacherEntity> {
    let id = teacher.id;
    let created = sqlx::query_as::<_, TeacherEntity>(
        "INSERT INTO teachers (id, first_name, last_name, date_of_birth, phone_number, image_url, homeroom_class_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *"
    )
    .bind(id)
    .bind(teacher.first_name)
    .bind(teacher.last_name)
    .bind(teacher.date_of_birth)
    .bind(teacher.phone_number)
    .bind(teacher.image_url)
    .bind(teacher.homeroom_class_id)
    .fetch_one(db)
    .await?;

    Ok(created)
}

pub async fn create_student(db: &Pool<Postgres>, student: StudentEntity) -> sqlx::Result<StudentEntity> {
    let id = student.id;
    let created = sqlx::query_as::<_, StudentEntity>(
        "INSERT INTO students (id, first_name, last_name, date_of_birth, phone_number, image_url, class_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *"
    )
    .bind(id)
    .bind(student.first_name)
    .bind(student.last_name)
    .bind(student.date_of_birth)
    .bind(student.phone_number)
    .bind(student.image_url)
    .bind(student.class_id)
    .fetch_one(db)
    .await?;

    Ok(created)
}