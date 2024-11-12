use sea_orm::prelude::Date;
use sea_orm::prelude::StringLen;
use sea_orm::sea_query::any;
use sea_orm::ActiveModelBehavior;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DbErr;
use sea_orm::DeriveEntityModel;
use sea_orm::DerivePrimaryKey;
use sea_orm::DeriveRelation;
use sea_orm::EntityTrait;
use sea_orm::EnumIter;
use sea_orm::PrimaryKeyTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "guests")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(column_type = "String(StringLen::N(32))")]
    pub first_name: String,

    #[sea_orm(column_type = "String(StringLen::N(32))")]
    pub last_name: String,

    pub date_of_birth: Date,

    #[sea_orm(column_type = "String(StringLen::N(16))", unique)]
    pub ucn: Option<String>,

    #[sea_orm(column_type = "String(StringLen::N(16))", unique)]
    pub id_card_number: Option<String>,

    #[sea_orm(column_type = "String(StringLen::N(32))")]
    pub id_card_issue_authority: Option<String>,

    pub id_card_issue_date: Option<Date>,

    pub id_card_validity: Option<Date>,

    #[sea_orm(column_type = "String(StringLen::N(16))", unique)]
    pub phone_number: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

pub async fn find_first_by_ucn_or_card_number_or_phone<T>(
    db: &T,
    ucn: &Option<String>,
    card_number: &Option<String>,
    phone_number: &Option<String>,
) -> Result<Option<Model>, DbErr>
where
    T: ConnectionTrait,
{
    let result = crate::persistence::guest::Entity::find()
        .filter(any![
            crate::persistence::guest::Column::Ucn
                .is_not_null()
                .and(crate::persistence::guest::Column::Ucn.eq(ucn.clone())),
            crate::persistence::guest::Column::IdCardNumber
                .is_not_null()
                .and(crate::persistence::guest::Column::IdCardNumber.eq(card_number.clone())),
            crate::persistence::guest::Column::PhoneNumber
                .is_not_null()
                .and(crate::persistence::guest::Column::PhoneNumber.eq(phone_number.clone())),
        ])
        .one(db)
        .await?;

    Ok(result)
}
