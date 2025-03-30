use anyhow::{Error, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{schema, training_setup::TrainingSetup};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::training)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Training {
    id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    hidden: i32,
    pub sets: i32,
    pub warmup_s: i32,
    pub exercise_s: i32,
    pub rest_s: i32,
}

impl Training {
    pub fn is_hidden(&self) -> bool {
        return self.hidden == 0;
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = match hidden {
            true => 1,
            false => 0,
        }
    }
}

impl From<Training> for TrainingSetup {
    fn from(value: Training) -> Self {
        Self {
            name: value.name,
            exercise_s: value.exercise_s as usize,
            rest_s: value.rest_s as usize,
            sets: value.sets as usize,
            prepare_s: value.warmup_s as usize,
        }
    }
}

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn open() -> Result<Database, Error> {
        dotenvy::dotenv()?;
        let database_url = std::env::var("DATABASE_URL")?;
        Ok(Database {
            connection: SqliteConnection::establish(&database_url)?,
        })
    }

    pub fn get_trainings(&mut self) -> Result<Vec<Training>, Error> {
        use schema::training::dsl::*;

        Ok(training
            .select(Training::as_select())
            .load(&mut self.connection)?)
    }
}
