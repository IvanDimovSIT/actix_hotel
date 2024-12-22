use sea_orm::prelude::DateTime;
use sea_orm::prelude::StringLen;
use sea_orm::sqlx::types::chrono::TimeZone;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::ActiveModelBehavior;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DbErr;
use sea_orm::DeriveEntityModel;
use sea_orm::DerivePrimaryKey;
use sea_orm::DeriveRelation;
use sea_orm::EntityTrait;
use sea_orm::EnumIter;
use sea_orm::IntoActiveModel;
use sea_orm::PrimaryKeyTrait;
use sea_orm::QueryFilter;

#[derive(Clone, Debug, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "invalidated_token")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "String(StringLen::N(512))"
    )]
    pub jwt: String,
    pub added: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn is_invalidated<T>(db: &T, token: &str) -> Result<bool, DbErr>
where
    T: ConnectionTrait,
{
    Ok(Entity::find_by_id(token).one(db).await?.is_some())
}

pub async fn invalidate<T>(db: &T, token: &str) -> Result<(), DbErr>
where
    T: ConnectionTrait,
{
    Model {
        jwt: token.to_owned(),
        added: Utc::now().naive_utc(),
    }
    .into_active_model()
    .insert(db)
    .await?;

    Ok(())
}

pub async fn remove_old<T>(db: &T, jwt_validity_secs: u64) -> Result<(), DbErr>
where
    T: ConnectionTrait,
{
    let now_ms = Utc::now().timestamp_millis();
    let max_creation_time = Utc
        .timestamp_opt(now_ms / 1000 - jwt_validity_secs as i64, 0)
        .unwrap();

    Entity::delete_many()
        .filter(Column::Added.lte(max_creation_time))
        .exec(db)
        .await?;

    Ok(())
}
