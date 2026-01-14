// src/souls/echo.rs
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SoulEcho {
    pub ancestor_id: Uuid,
    pub ancestor_name: String,
    pub trigger_type: EchoTrigger,
    pub message: String,
    pub memory_source: Uuid,
    pub visual_style: EchoVisual,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EchoTrigger {
    NearDeath,
    SameLocation,
    SameEnemy,
    ArtifactFound,
    CriticalDecision,
    Anniversary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoVisual {
    pub apparition_style: String,
    pub color_palette: String,
    pub intensity: f32,
}

pub struct EchoSystem;

#[derive(sqlx::FromRow)]
struct AncestorData {
    id: Uuid,
    name: String,
    death_location: Option<String>,
    killed_by: Option<String>,
}

#[derive(sqlx::FromRow)]
struct MemoryData {
    id: Uuid,
    title: String,
    description: Option<String>,
}

impl EchoSystem {
    pub async fn check_echo_triggers(
        pool: &PgPool,
        heir_id: Uuid,
        context: &EchoContext,
    ) -> Result<Option<SoulEcho>, sqlx::Error> {
        // REPLACED MACRO
        let ancestors: Vec<AncestorData> = sqlx::query_as(
            r#"
            SELECT s.id, s.name, s.death_location, s.killed_by
            FROM souls s
            WHERE s.id IN (
                SELECT ancestor_id FROM souls WHERE id = $1
                UNION
                SELECT ancestor_id FROM souls WHERE ancestor_id IN (
                    SELECT ancestor_id FROM souls WHERE id = $1
                )
            )
            AND s.status IN ('deceased', 'retired', 'ascended')
            "#
        )
        .bind(heir_id)
        .fetch_all(pool)
        .await?;

        for ancestor in ancestors {
            if let Some(echo) = Self::try_trigger(pool, &ancestor, context).await? {
                return Ok(Some(echo));
            }
        }

        Ok(None)
    }

    async fn try_trigger(
        pool: &PgPool,
        ancestor: &AncestorData,
        context: &EchoContext,
    ) -> Result<Option<SoulEcho>, sqlx::Error> {
        if context.heir_hp_percent < 0.1 {
            return Self::create_echo(
                pool,
                ancestor.id,
                EchoTrigger::NearDeath,
                "¡No te rindas! Yo también enfrenté la oscuridad...",
            ).await;
        }

        if let Some(ref location) = context.current_location {
            if ancestor.death_location.as_ref() == Some(location) {
                return Self::create_echo(
                    pool,
                    ancestor.id,
                    EchoTrigger::SameLocation,
                    &format!("Aquí caí... pero tú puedes triunfar donde yo fallé."),
                ).await;
            }
        }

        if let Some(ref enemy) = context.current_enemy {
            if ancestor.killed_by.as_ref() == Some(enemy) {
                return Self::create_echo(
                    pool,
                    ancestor.id,
                    EchoTrigger::SameEnemy,
                    &format!("Este ser me arrebató la vida. ¡Véngame, descendiente!"),
                ).await;
            }
        }

        Ok(None)
    }

    async fn create_echo(
        pool: &PgPool,
        ancestor_id: Uuid,
        trigger: EchoTrigger,
        message: &str,
    ) -> Result<Option<SoulEcho>, sqlx::Error> {
        // REPLACED MACRO
        let memory: Option<MemoryData> = sqlx::query_as(
            r#"
            SELECT id, title, description
            FROM memories
            WHERE soul_id = $1 AND is_echo_trigger = true
            ORDER BY emotional_weight DESC
            LIMIT 1
            "#
        )
        .bind(ancestor_id)
        .fetch_optional(pool)
        .await?;

        if let Some(mem) = memory {
            let ancestor_name: (String,) = sqlx::query_as(
                "SELECT name FROM souls WHERE id = $1"
            )
            .bind(ancestor_id)
            .fetch_one(pool)
            .await?;

            Ok(Some(SoulEcho {
                ancestor_id,
                ancestor_name: ancestor_name.0,
                trigger_type: trigger,
                message: message.to_string(),
                memory_source: mem.id,
                visual_style: EchoVisual {
                    apparition_style: "ethereal_ghost".to_string(),
                    color_palette: "ancestral_gold".to_string(),
                    intensity: 0.7,
                },
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn dm_trigger_echo(
        pool: &PgPool,
        _heir_id: Uuid,
        ancestor_id: Uuid,
        custom_message: Option<String>,
    ) -> Result<SoulEcho, sqlx::Error> {
        // REPLACED MACRO
        let ancestor: (String,) = sqlx::query_as(
            "SELECT name FROM souls WHERE id = $1"
        )
        .bind(ancestor_id)
        .fetch_one(pool)
        .await?;

        let memory: MemoryData = sqlx::query_as(
            r#"
            SELECT id, title, description
            FROM memories
            WHERE soul_id = $1 AND is_echo_trigger = true
            ORDER BY emotional_weight DESC
            LIMIT 1
            "#
        )
        .bind(ancestor_id)
        .fetch_one(pool)
        .await?;

        Ok(SoulEcho {
            ancestor_id,
            ancestor_name: ancestor.0,
            trigger_type: EchoTrigger::CriticalDecision,
            message: custom_message.unwrap_or(memory.description.unwrap_or(memory.title)),
            memory_source: memory.id,
            visual_style: EchoVisual {
                apparition_style: "dramatic_vision".to_string(),
                color_palette: "ethereal_blue".to_string(),
                intensity: 1.0,
            },
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct EchoContext {
    pub heir_hp_percent: f32,
    pub current_location: Option<String>,
    pub current_enemy: Option<String>,
    pub is_combat: bool,
}
