use std::fmt::Display;

use async_trait::async_trait;
use serde::Serialize;
use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::email::{Email, EmailKind},
    use_cases::email::EmailPersistence,
};

// User struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct EmailDb {
    pub id: Uuid,
    pub from_mail: String,
    pub to_mail: String,
    pub mail_subject: String,
    pub mail_body: String,
    pub email_kind: EmailKindDb,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<EmailDb> for Email {
    fn from(email_db: EmailDb) -> Self {
        Email {
            id: email_db.id,
            from_mail: email_db.from_mail,
            to_mail: email_db.to_mail,
            mail_subject: email_db.mail_subject,
            mail_body: email_db.mail_body,
            email_kind: email_db.email_kind.into(),
            created_at: email_db.created_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum EmailKindDb {
    #[default]
    Verification,
}

impl From<EmailKindDb> for EmailKind {
    fn from(value: EmailKindDb) -> Self {
        match value {
            EmailKindDb::Verification => EmailKind::Verification,
        }
    }
}

impl Display for EmailKindDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        EmailKind::from(*self).fmt(f)
    }
}

impl EmailKindDb {
    pub fn to_id(self) -> i32 {
        EmailKind::from(self).to_id()
    }

    pub fn from_id(id: i32) -> Option<Self> {
        EmailKind::from_id(id).map(|kind| match kind {
            EmailKind::Verification => EmailKindDb::Verification,
        })
    }
}

// Implement Type trait to tell SQLx how to handle this type
impl Type<Postgres> for EmailKindDb {
    fn type_info() -> PgTypeInfo {
        <i32 as Type<Postgres>>::type_info()
    }
}

// Implement Encode to convert enum to database value
impl<'q> Encode<'q, Postgres> for EmailKindDb {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <i32 as Encode<Postgres>>::encode_by_ref(&self.to_id(), buf)
    }
}

// Implement Decode to convert database value to enum
impl<'r> Decode<'r, Postgres> for EmailKindDb {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let id = <i32 as Decode<Postgres>>::decode(value)?;
        Self::from_id(id).ok_or_else(|| format!("Invalid email kind id: {id}").into())
    }
}

#[async_trait]
impl EmailPersistence for PostgresPersistence {
    async fn add_email(
        &self,
        from: String,
        to: String,
        subject: String,
        body: String,
        kind: EmailKind,
    ) -> AppResult<()> {
        let uuid = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO emails (id, from_mail, to_mail, mail_subject, mail_body, email_kind)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            uuid,
            from,
            to,
            subject,
            body,
            kind.to_id()
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }
}
