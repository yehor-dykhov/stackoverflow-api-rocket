use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        QuestionsDaoImpl {
            db
        }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record_result = sqlx::query!(
            r#"
            INSERT INTO questions ( title, description )
            VALUES ( $1, $2 )
            RETURNING *
            "#,
            question.title,
            question.description
        )
            .fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)));

        match record_result {
            Ok(record) => Ok(QuestionDetail {
                question_uuid: record.question_uuid.to_string(),
                title: record.title.clone(),
                description: record.description.clone(),
                created_at: record.created_at.to_string(),
            }),
            Err(e) => Err(DBError::Other(Box::new(e))),
        }
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(question_uuid.as_str())
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let record_result = sqlx::query!(
            "DELETE FROM questions WHERE question_uuid = $1",
            uuid
        )
            .execute(&self.db)
            .await;

        match record_result {
            Ok(..) => Ok(()),
            Err(e) => Err(DBError::Other(Box::new(e))),
        }
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let records_result = sqlx::query!("SELECT * FROM questions")
            .fetch_all(&self.db)
            .await;

        match records_result {
            Ok(records) => Ok(
                records.iter()
                    .map(|record| QuestionDetail {
                        question_uuid: record.question_uuid.to_string(),
                        title: record.title.clone(),
                        description: record.description.clone(),
                        created_at: record.created_at.to_string(),
                    })
                    .collect()
            ),
            Err(e) => Err(DBError::Other(Box::new(e)))
        }
    }
}
