use super::super::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::authz;
use db::queries::document_pipelines;
use db::Pool;
use serde::Deserialize;
use validator::Validate;
use web_pages::routes::document_pipelines::{Index, New};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewForm {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub dataset_id: i32,
}

pub async fn new(
    New { team_id }: New,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(new_pipeline): Form<NewForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    if new_pipeline.validate().is_ok() {
        let api_key: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        document_pipelines::insert()
            .bind(
                &transaction,
                &new_pipeline.dataset_id,
                &rbac.user_id,
                &team_id,
                &new_pipeline.name,
                &api_key,
            )
            .await?;
    }

    transaction.commit().await?;

    super::super::layout::redirect_and_snackbar(&Index { team_id }.to_string(), "Pipeline Created")
}
