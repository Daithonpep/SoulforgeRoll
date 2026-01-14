// src/souls/mod.rs
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

pub mod lineage;
pub mod echo;

// ═══════════════════════════════════════════════════════════════
// ESTRUCTURAS
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)] 
pub struct Soul {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub title: Option<String>,
    pub status: String, 
    pub age_years: i32, 
    pub age_category: String, 
    pub sessions_played: i32,
    pub character_data: serde_json::Value,
    pub lineage_id: Option<Uuid>,
    pub ancestor_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SoulStatus {
    Alive,
    Deceased,
    Retired,
    Missing,
    Ascended,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)] 
#[serde(rename_all = "lowercase")]
pub enum AgeCategory {
    Young,    // 16-25
    Adult,    // 26-40
    Mature,   // 41-55
    Elder,    // 56-70
    Ancient,  // 71+
}

// Helpers for string conversion
impl AgeCategory {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "young" => AgeCategory::Young,
            "adult" => AgeCategory::Adult,
            "mature" => AgeCategory::Mature,
            "elder" => AgeCategory::Elder,
            "ancient" => AgeCategory::Ancient,
            _ => AgeCategory::Adult, // Default fallback
        }
    }

    pub fn to_db_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeathEvent {
    pub cause: String,
    pub location: String,
    pub killed_by: Option<String>,
    pub last_words: Option<String>,
    pub witnesses: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgingResult {
    pub new_age: i32,
    pub category_changed: bool,
    pub old_category: AgeCategory,
    pub new_category: AgeCategory,
    pub stat_changes: StatChanges,
    pub triggered_event: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StatChanges {
    pub strength: i32,      
    pub dexterity: i32,     
    pub constitution: i32,  
    pub wisdom: i32,        
    pub intelligence: i32,  
    pub charisma: i32,      
}

// ═══════════════════════════════════════════════════════════════
// SISTEMA DE ENVEJECIMIENTO
// ═══════════════════════════════════════════════════════════════

pub struct AgingSystem;

impl AgingSystem {
    fn get_race_aging_config(race: &str) -> RaceAgingConfig {
        match race.to_lowercase().as_str() {
            "human" => RaceAgingConfig { years_per_session: 0.5, young_max: 25, adult_max: 40, mature_max: 55, elder_max: 75, max_lifespan: 90 },
            "elf" => RaceAgingConfig { years_per_session: 0.1, young_max: 100, adult_max: 300, mature_max: 500, elder_max: 700, max_lifespan: 900 },
            "dwarf" => RaceAgingConfig { years_per_session: 0.3, young_max: 40, adult_max: 100, mature_max: 200, elder_max: 300, max_lifespan: 400 },
            "halfling" => RaceAgingConfig { years_per_session: 0.4, young_max: 30, adult_max: 50, mature_max: 80, elder_max: 100, max_lifespan: 120 },
            _ => RaceAgingConfig { years_per_session: 0.5, young_max: 25, adult_max: 40, mature_max: 55, elder_max: 75, max_lifespan: 90 },
        }
    }

    pub async fn process_session_aging(
        pool: &PgPool,
        soul_id: Uuid,
    ) -> Result<AgingResult, sqlx::Error> {
        let soul: Soul = sqlx::query_as(
            r#"
            SELECT id, user_id, name, title, status, 
                   age_years, age_category,
                   sessions_played, character_data, lineage_id, ancestor_id
            FROM souls WHERE id = $1
            "#
        )
        .bind(soul_id)
        .fetch_one(pool)
        .await?;

        let race = soul.character_data
            .get("race")
            .and_then(|r| r.as_str())
            .unwrap_or("human");

        let config = Self::get_race_aging_config(race);
        
        let new_sessions = soul.sessions_played + 1;
        let age_increase = config.years_per_session;
        let new_age = soul.age_years + age_increase.ceil() as i32;
        
        // Convert string category to enum logic
        let current_category_enum = AgeCategory::from_string(&soul.age_category);
        let new_category_enum = Self::get_age_category(new_age, &config);
        
        let category_changed = new_category_enum != current_category_enum;
        
        let stat_changes = if category_changed {
            Self::calculate_stat_changes(&current_category_enum, &new_category_enum)
        } else {
            StatChanges::default()
        };

        let triggered_event = if new_age >= config.max_lifespan {
            Some("natural_death".to_string())
        } else if category_changed {
            Some(format!("became_{:?}", new_category_enum).to_lowercase())
        } else {
            None
        };

        sqlx::query(
            r#"
            UPDATE souls 
            SET age_years = $1, 
                age_category = $2,
                sessions_played = $3,
                last_session = NOW(),
                updated_at = NOW()
            WHERE id = $4
            "#
        )
        .bind(new_age)
        .bind(new_category_enum.to_db_string()) 
        .bind(new_sessions)
        .bind(soul_id)
        .execute(pool)
        .await?;

        if category_changed {
            Self::apply_stat_changes(pool, soul_id, &stat_changes).await?;
        }

        Ok(AgingResult {
            new_age,
            category_changed,
            old_category: current_category_enum,
            new_category: new_category_enum,
            stat_changes,
            triggered_event,
        })
    }

    fn get_age_category(age: i32, config: &RaceAgingConfig) -> AgeCategory {
        if age <= config.young_max { AgeCategory::Young }
        else if age <= config.adult_max { AgeCategory::Adult }
        else if age <= config.mature_max { AgeCategory::Mature }
        else if age <= config.elder_max { AgeCategory::Elder }
        else { AgeCategory::Ancient }
    }

    fn calculate_stat_changes(from: &AgeCategory, to: &AgeCategory) -> StatChanges {
        if matches!(from, AgeCategory::Young) && matches!(to, AgeCategory::Adult) {
            return StatChanges { strength: 1, constitution: 1, wisdom: 1, ..Default::default() };
        }
        if matches!(from, AgeCategory::Adult) && matches!(to, AgeCategory::Mature) {
            return StatChanges { strength: -1, wisdom: 2, intelligence: 1, charisma: 1, ..Default::default() };
        }
        if matches!(from, AgeCategory::Mature) && matches!(to, AgeCategory::Elder) {
            return StatChanges { strength: -2, dexterity: -1, constitution: -1, wisdom: 2, intelligence: 1, ..Default::default() };
        }
        if matches!(from, AgeCategory::Elder) && matches!(to, AgeCategory::Ancient) {
            return StatChanges { strength: -2, dexterity: -2, constitution: -2, wisdom: 3, intelligence: 2, charisma: 1, ..Default::default() };
        }
        StatChanges::default()
    }

    async fn apply_stat_changes(
        pool: &PgPool,
        soul_id: Uuid,
        changes: &StatChanges,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE souls
            SET character_data = jsonb_set(
                jsonb_set(
                    jsonb_set(
                        jsonb_set(
                            jsonb_set(
                                jsonb_set(
                                    character_data,
                                    '{stats,strength}',
                                    (COALESCE((character_data->'stats'->>'strength')::int, 10) + $1)::text::jsonb
                                ),
                                '{stats,dexterity}',
                                (COALESCE((character_data->'stats'->>'dexterity')::int, 10) + $2)::text::jsonb
                            ),
                            '{stats,constitution}',
                            (COALESCE((character_data->'stats'->>'constitution')::int, 10) + $3)::text::jsonb
                        ),
                        '{stats,wisdom}',
                        (COALESCE((character_data->'stats'->>'wisdom')::int, 10) + $4)::text::jsonb
                    ),
                    '{stats,intelligence}',
                    (COALESCE((character_data->'stats'->>'intelligence')::int, 10) + $5)::text::jsonb
                ),
                '{stats,charisma}',
                (COALESCE((character_data->'stats'->>'charisma')::int, 10) + $6)::text::jsonb
            ),
            updated_at = NOW()
            WHERE id = $7
            "#
        )
        .bind(changes.strength)
        .bind(changes.dexterity)
        .bind(changes.constitution)
        .bind(changes.wisdom)
        .bind(changes.intelligence)
        .bind(changes.charisma)
        .bind(soul_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}

pub struct RaceAgingConfig {
    years_per_session: f32,
    young_max: i32,
    adult_max: i32,
    mature_max: i32,
    elder_max: i32,
    max_lifespan: i32,
}

// ═══════════════════════════════════════════════════════════════
// SISTEMA DE MUERTE
// ═══════════════════════════════════════════════════════════════

pub struct DeathSystem;

#[derive(sqlx::FromRow)]
struct ArtifactShort {
    id: Uuid,
}

impl DeathSystem {
    pub async fn process_death(
        pool: &PgPool,
        soul_id: Uuid,
        death: DeathEvent,
    ) -> Result<DeathResult, sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query(
            r#"
            UPDATE souls
            SET status = 'deceased',
                death_date = NOW(),
                death_cause = $1,
                death_location = $2,
                killed_by = $3,
                last_words = $4,
                updated_at = NOW()
            WHERE id = $5
            "#
        )
        .bind(&death.cause)
        .bind(&death.location)
        .bind(&death.killed_by)
        .bind(&death.last_words)
        .bind(soul_id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO memories (soul_id, memory_type, title, description, location, emotional_weight, is_echo_trigger)
            VALUES ($1, 'loss', 'Muerte', $2, $3, 10, true)
            "#
        )
        .bind(soul_id)
        .bind(format!("Cayó en {}. {}", death.location, death.cause))
        .bind(&death.location)
        .execute(&mut *tx)
        .await?;

        let artifacts: Vec<ArtifactShort> = sqlx::query_as(
            r#"
            SELECT id FROM artifacts WHERE bound_to = $1
            "#
        )
        .bind(soul_id)
        .fetch_all(&mut *tx)
        .await?;

        for artifact in &artifacts {
            Self::add_to_artifact_history(&mut tx, artifact.id, soul_id, true).await?;
            
            sqlx::query(
                r#"
                UPDATE artifacts
                SET bound_to = NULL,
                    current_location = 'lost',
                    updated_at = NOW()
                WHERE id = $1
                "#
            )
            .bind(artifact.id)
            .execute(&mut *tx)
            .await?;

            sqlx::query(
                r#"
                INSERT INTO artifact_memories 
                (artifact_id, soul_id, event_type, description, location, karma_change)
                VALUES ($1, $2, 'owner_died', $3, $4, -1)
                "#
            )
            .bind(artifact.id)
            .bind(soul_id)
            .bind("Su portador cayó en batalla")
            .bind(&death.location)
            .execute(&mut *tx)
            .await?;
        }

        sqlx::query(
            r#"
            UPDATE lineages
            SET fallen_heroes = fallen_heroes + 1,
                living_members = living_members - 1
            WHERE id = (SELECT lineage_id FROM souls WHERE id = $1)
            "#
        )
        .bind(soul_id)
        .execute(&mut *tx)
        .await?;

        for witness_id in &death.witnesses {
            sqlx::query(
                r#"
                INSERT INTO memories (soul_id, memory_type, title, description, location, emotional_weight, witnesses)
                VALUES ($1, 'loss', $2, $3, $4, 8, $5)
                "#
            )
            .bind(witness_id)
            .bind("La caída de un compañero")
            .bind(format!("Presenciaste la muerte de un aliado en {}", death.location))
            .bind(&death.location)
            .bind(vec![soul_id])
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        let row: (String, Option<String>, i32, i32, i32) = sqlx::query_as(
            r#"
            SELECT name, title, age_years, sessions_played, kills_total
            FROM souls WHERE id = $1
            "#
        )
        .bind(soul_id)
        .fetch_one(pool)
        .await?;

        Ok(DeathResult {
            soul_id,
            name: row.0,
            title: row.1,
            age_at_death: row.2,
            sessions_lived: row.3,
            total_kills: row.4,
            artifacts_left_behind: artifacts.len() as i32,
            can_create_heir: true,
        })
    }

    pub async fn process_retirement(
        pool: &PgPool,
        soul_id: Uuid,
        retirement_location: &str,
        final_title: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE souls
            SET status = 'retired',
                title = $1,
                death_location = $2,
                updated_at = NOW()
            WHERE id = $3
            "#
        )
        .bind(final_title)
        .bind(retirement_location)
        .bind(soul_id)
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO memories (soul_id, memory_type, title, description, location, emotional_weight, is_echo_trigger)
            VALUES ($1, 'blessing', 'Retiro Honorable', $2, $3, 9, true)
            "#
        )
        .bind(soul_id)
        .bind(format!("Después de una vida de aventuras, se retiró como {}", final_title))
        .bind(retirement_location)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn add_to_artifact_history(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        artifact_id: Uuid,
        soul_id: Uuid,
        died_holding: bool,
    ) -> Result<(), sqlx::Error> {
        let soul: (String, i32) = sqlx::query_as(
            "SELECT name, kills_total FROM souls WHERE id = $1"
        )
        .bind(soul_id)
        .fetch_one(&mut **tx)
        .await?;

        sqlx::query(
            r#"
            UPDATE artifacts
            SET previous_owners = previous_owners || $1::jsonb,
                updated_at = NOW()
            WHERE id = $2
            "#
        )
        .bind(serde_json::json!([{
            "soul_id": soul_id,
            "name": soul.0,
            "kills_with_item": soul.1,
            "died_while_holding": died_holding
        }]))
        .bind(artifact_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct DeathResult {
    pub soul_id: Uuid,
    pub name: String,
    pub title: Option<String>,
    pub age_at_death: i32,
    pub sessions_lived: i32,
    pub total_kills: i32,
    pub artifacts_left_behind: i32,
    pub can_create_heir: bool,
}
