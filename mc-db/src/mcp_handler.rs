use crate::DBClient;
use crate::model::McpServers;
use std::sync::Arc;

pub struct McpDBHandler {
    client: Arc<DBClient>,
}

impl McpDBHandler {
    pub fn new(client: Arc<DBClient>) -> Self {
        McpDBHandler { client }
    }

    pub async fn list_all(&self) -> Result<Vec<McpServers>, sqlx::Error> {
        sqlx::query_as::<_, McpServers>("SELECT * FROM tb_mcp_servers ORDER BY id")
            .fetch_all(&self.client.pool)
            .await
    }

    pub async fn list_with_limit(
        &self,
        name: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<McpServers>, sqlx::Error> {
        let query = match name {
            Some(name) => sqlx::query_as::<_, McpServers>(
                "SELECT * FROM tb_mcp_servers WHERE name ILIKE $1 ORDER BY id LIMIT $2 OFFSET $3",
            )
            .bind(format!("%{name}%")),
            None => sqlx::query_as::<_, McpServers>(
                "SELECT * FROM tb_mcp_servers ORDER BY id LIMIT $1 OFFSET $2",
            ),
        };

        query
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.client.pool)
            .await
    }

    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tb_mcp_servers")
            .fetch_one(&self.client.pool)
            .await?;
        Ok(count)
    }

    pub async fn create(&self, server: &McpServers) -> Result<McpServers, sqlx::Error> {
        let res = if server.extra.is_some() {
            sqlx::query_as::<_, McpServers>(
                r#"
        INSERT INTO tb_mcp_servers
            (id, name, tag, endpoint, transport_type, description, create_from, extra)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
            )
            .bind(server.id)
            .bind(&server.name)
            .bind(&server.tag)
            .bind(&server.endpoint)
            .bind(&server.transport_type)
            .bind(&server.description)
            .bind(&server.create_from)
            .bind(&server.extra)
            .fetch_one(&self.client.pool)
            .await?
        } else {
            sqlx::query_as::<_, McpServers>(
                r#"
        INSERT INTO tb_mcp_servers
            (id,name, tag, endpoint, transport_type, description, create_from)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
            )
            .bind(server.id)
            .bind(&server.name)
            .bind(&server.tag)
            .bind(&server.endpoint)
            .bind(&server.transport_type)
            .bind(&server.description)
            .bind(&server.create_from)
            .fetch_one(&self.client.pool)
            .await?
        };

        Ok(res)
    }

    pub async fn update(&self, server: McpServers) -> Result<McpServers, sqlx::Error> {
        let query = match server.extra {
            None => sqlx::query_as::<_, McpServers>(
                r#"
                    UPDATE tb_mcp_servers
                    SET
                        name = $1,
                        tag = $2,
                        endpoint = $3,
                        transport_type = $4,
                        create_from = $5,
                        description = $6,
                        updated_at = CURRENT_TIMESTAMP,
                        disabled = $7
                    WHERE id = $8
                    RETURNING *;"#,
            ),
            Some(extra) => sqlx::query_as::<_, McpServers>(
                r#"
                    UPDATE tb_mcp_servers
                    SET
                        extra = $1,
                        name = $2,
                        tag = $3,
                        endpoint = $4,
                        transport_type = $5,
                        create_from = $6,
                        description = $7,
                        updated_at = CURRENT_TIMESTAMP,
                        disabled = $8
                    WHERE id = $9
                    RETURNING *;"#,
            )
            .bind(extra),
        };

        let server = query
            .bind(server.name)
            .bind(server.tag)
            .bind(server.endpoint)
            .bind(server.transport_type)
            .bind(server.create_from)
            .bind(server.description)
            .bind(server.disabled)
            .bind(server.id)
            .fetch_one(&self.client.pool)
            .await?;

        Ok(server)
    }
}
