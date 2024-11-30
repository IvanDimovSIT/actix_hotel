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
use sea_orm::FromQueryResult;
use sea_orm::PrimaryKeyTrait;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::Related;
use sea_orm::RelationDef;
use sea_orm::RelationTrait;
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
pub enum Relation {
    #[sea_orm(has_many = "super::booking::Entity")]
    Booking,
    #[sea_orm(has_many = "super::booking_guest::Entity")]
    BookingGuest,
}
impl Related<super::booking_guest::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingGuest.def()
    }
}

impl Related<super::booking::Entity> for Entity {
    fn to() -> RelationDef {
        super::booking_guest::Relation::Booking.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::booking_guest::Relation::Guest.def().rev())
    }
}

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

pub async fn find_all_ids_by_criteria<T>(
    db: &T,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    date_of_birth: Option<Date>,
    ucn: Option<String>,
) -> Result<Vec<Uuid>, DbErr>
where
    T: ConnectionTrait,
{
    #[derive(Debug, FromQueryResult)]
    struct GuestId{
        g_id: Uuid
    }

    let mut query = crate::persistence::guest::Entity::find()
        .select_only()
        .column_as(Column::Id, "g_id");
    
    if let Some(some) = first_name {
        query = query.filter(crate::persistence::guest::Column::FirstName.eq(some))
    }
    if let Some(some) = last_name {
        query = query.filter(crate::persistence::guest::Column::LastName.eq(some))
    }
    if let Some(some) = phone_number {
        query = query.filter(crate::persistence::guest::Column::PhoneNumber.eq(some))
    }
    if let Some(some) = date_of_birth {
        query = query.filter(crate::persistence::guest::Column::DateOfBirth.eq(some))
    }
    if let Some(some) = ucn {
        query = query.filter(crate::persistence::guest::Column::Ucn.eq(some))
    }

    let result = query
        .into_model::<GuestId>()
        .all(db)
        .await?
        .into_iter()
        .map(|guest| guest.g_id)
        .collect();

    Ok(result)
}