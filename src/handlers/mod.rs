use rocket::serde::json::Json;

use crate::models::*;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(question: Json<Question>) -> Json<QuestionDetail> {
    Json(QuestionDetail {
        question_uuid: "question_uuid".to_string(),
        title: "title".to_string(),
        description: "description".to_string(),
        created_at: "created_at".to_string(),
    })
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    Json(vec![QuestionDetail {
        question_uuid: "question_uuid".to_string(),
        title: "title".to_string(),
        description: "description".to_string(),
        created_at: "created_at".to_string(),
    }])
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(question_uuid: Json<QuestionId>) {
    ()
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(answer: Json<Answer>) -> Json<AnswerDetail> {
    Json(AnswerDetail {
        answer_uuid: "question_uuid".to_string(),
        question_uuid: "question_uuid".to_string(),
        content: "title".to_string(),
        created_at: "created_at".to_string(),
    })
}

#[get("/answers")]
pub async fn read_answers() -> Json<Vec<AnswerDetail>> {
    Json(vec![AnswerDetail {
        answer_uuid: "question_uuid".to_string(),
        question_uuid: "question_uuid".to_string(),
        content: "title".to_string(),
        created_at: "created_at".to_string(),
    }])
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(answer_uuid: Json<AnswerId>) {
    ()
}
