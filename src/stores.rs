use crate::types::{
    answer::Answer, question::NewQuestion, question::Question, question::QuestionID,
};

use handle_errors::CustomError;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}
impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(_e) => panic!("Couldn't establish DB connection!"),
        };
        Store {
            connection: db_pool,
        }
    }
    pub async fn get_all_questions(
        self,
        offset: i32,
        limit: Option<i32>,
    ) -> Result<Vec<Question>, CustomError> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionID(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(CustomError::DatabaseQueryError)
            }
        }
    }
    pub async fn get_a_question(self, id: i32) -> Result<Question, CustomError> {
        match sqlx::query("SELECT * FROM questions WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| Question {
                id: QuestionID(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(CustomError::DatabaseQueryError)
            }
        }
    }
    pub async fn add_question(self, new_question: NewQuestion) -> Result<Question, CustomError> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3) RETURNING id, title, content, tags",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .map(|row: PgRow| Question {
            id: QuestionID(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(CustomError::DatabaseQueryError)
            }
        }
    }

    pub async fn update_question(
        self,
        question: Question,
        id: i32,
    ) -> Result<Question, CustomError> {
        match sqlx::query(
            "UPDATE questions SET title = $1, content = $2, tags = $3 WHERE id = $4 RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(id)
        .map(|row: PgRow| Question {
            id: QuestionID(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(CustomError::DatabaseQueryError)
            }
        }
    }
    pub async fn delete_question(self, id: i32) -> Result<bool, CustomError> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(CustomError::DatabaseQueryError)
            }
        }
    }

    pub async fn add_answer(self, answer: Answer) -> Result<bool, CustomError> {
        match sqlx::query("INSERT INTO answers (content, corresponding_question) VALUES ($1, $2) RETURNING content, corresponding_question")
            .bind(answer.content)
            .bind(answer.question_id)
            .map(|row: PgRow| Answer {
                content: row.get("content"),
                question_id: row.get("corresponding_question"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(_answer) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(CustomError::DatabaseQueryError)
            }
        }
    }
}
