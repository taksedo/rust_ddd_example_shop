#![allow(non_snake_case)]

use common::types::base::{AM, ArcMutexTrait};
use diesel_migrations::MigrationHarness;
use domain::test_fixtures::*;
use postgres_persistence::{
    database_start::MIGRATIONS, postgres_meal_repository::PostgresMealRepository,
};
use usecase::menu::access::{meal_extractor::MealExtractor, meal_persister::MealPersister};

use crate::test_fixtures::{MockEventPublisher, TestDb, rnd_new_meal_with_meal_id};

mod test_fixtures;

#[test]
fn get_by_id__not_found() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, AM::new_am(MockEventPublisher::default()));

    let result = repository.get_by_id(&rnd_meal_id());

    assert!(result.is_none())
}

#[test]
fn get_by_id__successfully_returned() {
    let meal = rnd_new_meal_with_meal_id(rnd_meal_id());
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, AM::new_am(MockEventPublisher::default()));
    repository.save(meal.clone());

    let meal_id = *meal.id();
    let result = repository.get_by_id(&meal_id);

    assert!(result.is_some());
    assert_eq!(result.unwrap(), meal);
}

#[test]
fn get_by_name__not_found() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, AM::new_am(MockEventPublisher::default()));

    let result = repository.get_by_name(&rnd_meal_name());

    assert!(result.is_none());
}

#[test]
fn get_by_name__successfully_returned() {
    let meal = rnd_new_meal_with_meal_id(rnd_meal_id());
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, AM::new_am(MockEventPublisher::default()));
    repository.save(meal.clone());

    let meal_name = meal.name();
    let result = repository.get_by_name(meal_name);

    assert!(result.is_some());
    assert_eq!(result.unwrap(), meal);
}

#[test]
fn get_all__table_is_empty() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, AM::new_am(MockEventPublisher::default()));

    let result = repository.get_all();

    assert!(result.is_empty());
}

#[test]
fn get_all__table_is_not_empty() {
    let meal = rnd_new_meal_with_meal_id(rnd_meal_id());
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, AM::new_am(MockEventPublisher::default()));
    repository.save(meal.clone());

    let result = repository.get_all();

    assert!(!result.is_empty());
    assert_eq!(result.first().unwrap(), &meal);
}

#[test]
fn get_all__table_is_not_empty_but_removed() {
    let mut meal = rnd_new_meal_with_meal_id(rnd_meal_id());
    meal.remove_meal_from_menu();
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, AM::new_am(MockEventPublisher::default()));
    repository.save(meal.clone());

    let result = repository.get_all();

    assert!(result.is_empty());
}
