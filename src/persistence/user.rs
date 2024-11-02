use std::error::Error;

use sea_orm::prelude::StringLen;
use sea_orm::ActiveModelBehavior;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DeriveActiveEnum;
use sea_orm::DeriveEntityModel;
use sea_orm::DerivePrimaryKey;
use sea_orm::DeriveRelation;
use sea_orm::EntityTrait;
use sea_orm::EnumIter;
use sea_orm::PrimaryKeyTrait;
use sea_orm::QueryFilter;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub salt: String,
    pub password: String,
    pub role: Role,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Default, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(16))")]
pub enum Role {
    #[sea_orm(string_value = "User")]
    #[default]
    User,
    #[sea_orm(string_value = "Admin")]
    Admin,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

pub async fn find_user_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<Model>, Box<dyn Error>> {
    let user = crate::persistence::user::Entity::find()
        .filter(crate::persistence::user::Column::Email.eq(email))
        .one(db)
        .await?;

    Ok(user)
}

pub async fn find_user_by_id(
    db: &DatabaseConnection,
    id: &Uuid,
) -> Result<Option<Model>, Box<dyn Error>> {
    Ok(crate::persistence::user::Entity::find_by_id(*id)
        .one(db)
        .await?)
}
