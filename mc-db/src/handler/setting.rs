use crate::DBClient;
use crate::model::{SettingKey, SystemSettings};
use std::sync::Arc;

pub struct SystemSettingsDBHandler {
    client: Arc<DBClient>,
}

impl SystemSettingsDBHandler {
    pub fn new(client: Arc<DBClient>) -> Self {
        SystemSettingsDBHandler { client }
    }

    pub async fn list_all(&self) -> Result<Vec<SystemSettings>, sqlx::Error> {
        sqlx::query_as::<_, SystemSettings>("SELECT * FROM tb_system_settings ORDER BY setting_name")
            .fetch_all(&self.client.pool)
            .await
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Option<SystemSettings>, sqlx::Error> {
        sqlx::query_as::<_, SystemSettings>(
            "SELECT * FROM tb_system_settings WHERE setting_name = $1",
        )
            .bind(name)
            .fetch_optional(&self.client.pool)
            .await
    }

    pub async fn upsert(&self, name: &str, value: &str) -> Result<SystemSettings, sqlx::Error> {
        sqlx::query_as::<_, SystemSettings>(
            r#"
        INSERT INTO tb_system_settings (setting_name, setting_value)
        VALUES ($1, $2)
        ON CONFLICT (setting_name)
            DO UPDATE SET setting_value = EXCLUDED.setting_value
        RETURNING *
        "#,
        )
            .bind(name)
            .bind(value)
            .fetch_one(&self.client.pool)
            .await
    }

    pub async fn get_system_settings(&self, key: SettingKey) -> String {
        if let Ok(settings) = sqlx::query_as::<_, SystemSettings>(
            "SELECT * FROM tb_system_settings where setting_name = $1",
        )
            .bind(key.to_string())
            .fetch_one(&self.client.pool)
            .await
        {
            settings.setting_value.trim_end_matches('/').to_string()
        } else {
            String::from("http://127.0.0.1")
        }
    }
}
