use async_trait::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswersDaoImpl { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = Uuid::parse_str(answer.question_uuid.as_str())
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let record = sqlx::query!(
            r#"
            INSERT INTO answers ( question_uuid, content )
            VALUES ( $1, $2 )
            RETURNING *
            "#,
            uuid,
            answer.content
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e: sqlx::Error| {
            return match e {
                sqlx::Error::Database(e) => {
                    if let Some(code) = e.code() {
                        if code.eq(postgres_error_codes::FOREIGN_KEY_VIOLATION) {
                            return DBError::InvalidUUID(format!(
                                "Invalid uuid {}",
                                answer.question_uuid
                            ));
                        }
                    }
                    DBError::Other(Box::new(e))
                }
                e => return DBError::Other(Box::new(e)),
            };
        })?;

        Ok(AnswerDetail {
            question_uuid: record.question_uuid.to_string(),
            answer_uuid: record.answer_uuid.to_string(),
            content: record.content.clone(),
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid =
            Uuid::parse_str(&answer_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let record_result = sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&self.db)
            .await;

        match record_result {
            Ok(_) => Ok(()),
            Err(e) => Err(DBError::Other(Box::new(e))),
        }
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid =
            Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let records_result = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await;

        match records_result {
            Ok(records) => Ok(records
                .iter()
                .map(|record| {
                    return AnswerDetail {
                        answer_uuid: record.answer_uuid.to_string(),
                        question_uuid: record.question_uuid.to_string(),
                        content: record.content.clone(),
                        created_at: record.created_at.to_string(),
                    };
                })
                .collect()),
            Err(e) => Err(DBError::Other(Box::new(e))),
        }
    }
}
