use std::error::Error;

use sqlx::{
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};

use super::CountryCode;

impl Type<Postgres> for CountryCode {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("varchar")
    }
}

impl<'r> Encode<'r, Postgres> for CountryCode {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn Error + Send + Sync>> {
        <&str as Encode<Postgres>>::encode(self.as_ref(), buf)
    }
}

impl<'r> Decode<'r, Postgres> for CountryCode {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let s = <String as Decode<Postgres>>::decode(value)?;
        Ok(s.parse()?)
    }
}

#[cfg(test)]
#[cfg(not(feature = "ci"))]
mod tests {
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test]
    async fn encode(pool: PgPool) -> sqlx::Result<()> {
        let country_code: CountryCode = "AR".parse().unwrap();
        let encoded = sqlx::query_scalar!("SELECT $1::varchar", country_code as _)
            .fetch_one(&pool)
            .await?
            .unwrap();
        assert_eq!(encoded, "AR");
        Ok(())
    }

    #[sqlx::test]
    async fn decode(pool: PgPool) -> sqlx::Result<()> {
        let country_code: CountryCode = "AR".parse().unwrap();
        let decoded = sqlx::query_scalar!(r#"SELECT 'AR'::varchar as "val: CountryCode""#)
            .fetch_one(&pool)
            .await?
            .unwrap();
        assert_eq!(decoded, country_code);
        Ok(())
    }

    #[sqlx::test]
    async fn decode_error(pool: PgPool) -> sqlx::Result<()> {
        assert!(
            sqlx::query_scalar!(r#"SELECT 'BAD'::varchar as "val: CountryCode""#)
                .fetch_one(&pool)
                .await
                .is_err()
        );
        Ok(())
    }
}
