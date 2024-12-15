use sea_orm::prelude::StringLen;
use sea_orm::ActiveModelBehavior;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
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
    #[sea_orm(unique, db_type = "String(StringLen::N(255))")]
    pub email: String,
    #[sea_orm(db_type = "String(StringLen::N(255))")]
    pub password: String,
    pub role: Role,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Default, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema
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
pub enum Relation {
    #[sea_orm(has_one = "super::one_time_password::Entity")]
    OneTimePassword,
    #[sea_orm(has_one = "super::booking::Entity")]
    AdminBooking,
    #[sea_orm(has_one = "super::booking::Entity")]
    UserBooking,
}
impl ActiveModelBehavior for ActiveModel {}

pub async fn find_user_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<Model>, DbErr> {
    let user = crate::persistence::user::Entity::find()
        .filter(crate::persistence::user::Column::Email.eq(email))
        .one(db)
        .await?;

    Ok(user)
}

pub async fn find_user_by_id<T>(db: &T, id: &Uuid) -> Result<Option<Model>, DbErr>
where
    T: ConnectionTrait,
{
    Ok(crate::persistence::user::Entity::find_by_id(*id)
        .one(db)
        .await?)
}
