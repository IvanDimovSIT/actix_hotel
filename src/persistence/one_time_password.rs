use std::error::Error;

use sea_orm::prelude::DateTime;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DerivePrimaryKey;
use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;
use sea_orm::QueryFilter;
use sea_orm::Related;
use sea_orm::RelationDef;
use sea_orm::RelationTrait;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "otps")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub user_id: Uuid,
    #[sea_orm(db_type = "String(StringLen::N(32))")]
    pub otp_code: String,
    pub validity: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}
impl ActiveModelBehavior for ActiveModel {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

pub async fn delete_all_for_user(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<u64, Box<dyn Error>> {
    let result = crate::persistence::one_time_password::Entity::delete_many()
        .filter(crate::persistence::one_time_password::Column::UserId.eq(*user_id))
        .exec(db)
        .await?;

    Ok(result.rows_affected)
}
