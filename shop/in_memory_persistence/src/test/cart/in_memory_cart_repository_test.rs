use std::sync::{Arc, Mutex};

use domain::{
    main::cart::cart_events::{CartEventEnum, MealAddedToCartDomainEvent},
    test_fixtures::{rnd_cart, rnd_cart_with_customer_id, rnd_customer_id},
};
use usecase::main::cart::access::{
    cart_extractor::CartExtractor, cart_persister::CartPersister, cart_remover::CartRemover,
};

use crate::{
    main::cart::in_memory_cart_repository::InMemoryCartRepository,
    test_fixtures::{cart_with_events, TestEventPublisher},
};

#[test]
fn saving_cart_cart_doesnt_exist() {
    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryCartRepository::new(event_publisher.clone());
    let cart = cart_with_events();

    repository.save(cart.clone());

    let stored_cart = repository.storage.get(cart.for_customer()).unwrap();
    assert_eq!(stored_cart, &cart);
    assert_eq!(event_publisher.lock().unwrap().storage.len(), 1);

    let binding = event_publisher.lock().unwrap();
    let event: &CartEventEnum = binding.storage.first().unwrap();
    let event_struct: MealAddedToCartDomainEvent = event.clone().try_into().unwrap();
    assert_eq!(event_struct.cart_id, *cart.id());
}

#[test]
fn saving_cart_cart_exists() {
    let customer_id = rnd_customer_id();
    let existing_cart = rnd_cart_with_customer_id(customer_id);

    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryCartRepository::new(event_publisher.clone());
    repository.storage.insert(customer_id, existing_cart);

    let updated_cart = cart_with_events();
    repository.save(updated_cart.clone());
    repository.storage.insert(customer_id, updated_cart.clone());

    let binding = event_publisher.lock().unwrap();
    let event: &CartEventEnum = binding.storage.first().unwrap();
    let event_struct: Result<MealAddedToCartDomainEvent, _> = event.clone().try_into();
    assert!(event_struct.is_ok());
    assert_eq!(event_struct.unwrap().cart_id, *updated_cart.id());
}

#[test]
fn get_by_id_cart_exists() {
    let customer_id = rnd_customer_id();
    let existing_cart = rnd_cart_with_customer_id(customer_id);

    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryCartRepository::new(event_publisher.clone());
    repository
        .storage
        .insert(customer_id, existing_cart.clone());

    let cart = repository.get_cart(&customer_id);

    assert!(cart.is_some());

    let cart = cart.unwrap();
    assert_eq!(cart, existing_cart);
    assert_eq!(cart.id(), existing_cart.id());
    assert_eq!(cart.for_customer(), existing_cart.for_customer());
    assert_eq!(cart.created(), existing_cart.created());
    assert_eq!(cart.meals(), existing_cart.meals());
}

#[test]
fn get_by_id_cart_doesnt_exist() {
    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryCartRepository::new(event_publisher.clone());
    let cart = repository.get_cart(&rnd_customer_id());

    assert!(cart.is_none());
}

#[test]
fn delete_cart_cart_exists() {
    let existing_cart = rnd_cart();
    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryCartRepository::new(event_publisher.clone());
    repository
        .storage
        .insert(*existing_cart.for_customer(), existing_cart.clone());

    repository.delete_cart(existing_cart);
    assert!(repository.storage.is_empty());
}

#[test]
fn copy_cart_test() {
    let src = cart_with_events();
    assert!(!src.meals().is_empty());

    let copy = src.clone();

    assert_eq!(src, copy);
    assert_eq!(src.id(), copy.id());
    assert_eq!(src.for_customer(), copy.for_customer());
    assert_eq!(src.created(), copy.created());
    assert_eq!(src.meals(), copy.meals());
}
