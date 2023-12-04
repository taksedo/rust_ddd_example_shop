use std::any::Any;
use std::collections::HashMap;

use common::types::main::base::domain_entity::DomainEntityTrait;
use common::types::main::common::count::Count;
use derive_new::new;

use domain::main::cart::cart::Cart;
use domain::main::cart::value_objects::cart_id::CartId;
use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_events::MealEventEnum;
use domain::main::menu::meal_events::MealRemovedFromMenuDomainEvent;
use domain::main::menu::value_objects::meal_description::MealDescription;
use domain::main::menu::value_objects::meal_id::MealId;
use domain::main::menu::value_objects::meal_name::MealName;
use domain::main::menu::value_objects::price::Price;
use domain::main::order::shop_order::{OrderState, ShopOrder};
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use domain::test_fixtures::{order_with_state, rnd_meal};

use crate::main::cart::access::cart_extractor::CartExtractor;
use crate::main::cart::access::cart_persister::CartPersister;
use crate::main::cart::access::cart_remover::CartRemover;
use crate::main::menu::access::meal_extractor::MealExtractor;
use crate::main::menu::access::meal_persister::MealPersister;
use crate::main::order::access::shop_order_extractor::ShopOrderExtractor;

pub fn removed_meal() -> Meal {
    let mut meal = rnd_meal();
    meal.remove_meal_from_menu();
    meal
}

pub fn order_ready_for_pay() -> ShopOrder {
    order_with_state(OrderState::new_waiting_for_payment())
}

pub fn order_not_ready_for_pay() -> ShopOrder {
    order_with_state(OrderState::new_completed())
}

pub fn order_ready_for_cancel() -> ShopOrder {
    order_with_state(OrderState::new_paid())
}

pub fn order_not_ready_for_cancel() -> ShopOrder {
    order_with_state(OrderState::new_completed())
}

pub fn order_ready_for_confirm() -> ShopOrder {
    order_with_state(OrderState::new_paid())
}

pub fn order_not_ready_for_confirm() -> ShopOrder {
    order_with_state(OrderState::new_waiting_for_payment())
}

pub fn order_ready_for_complete() -> ShopOrder {
    order_with_state(OrderState::new_confirmed())
}

pub fn order_not_ready_for_complete() -> ShopOrder {
    order_with_state(OrderState::new_cancelled())
}

pub fn active_order() -> ShopOrder {
    order_with_state(OrderState::new_confirmed())
}

pub fn non_active_order() -> ShopOrder {
    order_with_state(OrderState::new_cancelled())
}

#[derive(new, Debug, Clone)]
pub struct MockMealPersister {
    #[new(value = "None")]
    pub meal: Option<Meal>,
}

impl MockMealPersister {
    pub fn verify_invoked(
        &self,
        id: Option<&MealId>,
        name: Option<&MealName>,
        description: Option<&MealDescription>,
        price: Option<&Price>,
    ) {
        let meal = &self.meal.clone().unwrap();
        if id.is_some() {
            assert_eq!(&meal.entity_params.id, id.unwrap())
        }
        if name.is_some() {
            assert_eq!(&meal.name, name.unwrap())
        }
        if description.is_some() {
            assert_eq!(&meal.description, description.unwrap())
        }
        if price.is_some() {
            assert_eq!(&meal.price, price.unwrap())
        }
    }

    pub fn verify_invoked_meal(&self, meal: Option<&Meal>) {
        if meal.is_some() {
            assert_eq!(&self.meal.clone().unwrap(), meal.unwrap())
        }
    }

    pub fn verify_events_after_deletion(&mut self, id: &MealId) {
        let event_enum: MealEventEnum = MealRemovedFromMenuDomainEvent::new(*id).into();
        let events = self
            .to_owned()
            .meal
            .unwrap()
            .entity_params
            .pop_events()
            .get(0)
            .unwrap()
            .clone();
        assert_eq!(events.type_id(), event_enum.type_id());
    }

    pub fn verify_empty(&self) {
        assert!(&self.meal.is_none());
    }
}

impl MealPersister for MockMealPersister {
    fn save(&mut self, meal: Meal) {
        self.meal = Some(meal);
    }
}

#[derive(new, Clone, PartialEq, Debug, Default)]
pub struct MockMealExtractor {
    #[new(default)]
    pub meal: Option<Meal>,
    #[new(default)]
    pub id: Option<MealId>,
    #[new(default)]
    pub name: Option<MealName>,
    #[new(default)]
    pub all: bool,
}

impl MealExtractor for MockMealExtractor {
    fn get_by_id(&mut self, id: MealId) -> Option<Meal> {
        self.id = Option::from(id);
        if Some(&self.meal).is_some() && self.id == Some(id) {
            self.clone().meal
        } else {
            None
        }
    }

    fn get_by_name(&mut self, name: MealName) -> Option<Meal> {
        self.name = Option::from(name.to_owned());
        if Some(&self.meal).is_some() && self.to_owned().name.unwrap() == name {
            self.to_owned().meal
        } else {
            None
        }
    }

    fn get_all(&mut self) -> Vec<Meal> {
        self.all = true;
        if self.meal.is_some() {
            vec![self.to_owned().meal.unwrap()]
        } else {
            vec![]
        }
    }
}

impl MockMealExtractor {
    pub fn verify_invoked_get_by_id(&self, id: &MealId) {
        assert_eq!(&self.id.unwrap(), id);
        assert!(!&self.all);
        assert!(&self.name.is_none());
    }

    pub fn verify_invoked_get_by_name(&self, name: &MealName) {
        assert_eq!(&self.clone().name.unwrap(), name);
        assert!(!&self.all);
        assert!(&self.id.is_none());
    }

    pub fn verify_invoked_get_all(&self) {
        assert!(&self.all);
        assert!(&self.id.is_none());
        assert!(&self.name.is_none());
    }

    pub fn verify_empty(&self) {
        assert!(&self.name.is_none());
    }
}

impl dyn MealExtractor + 'static {
    pub fn downcast_ref<T: MealExtractor + 'static>(&self) -> Option<&T> {
        unsafe { Some(&*(self as *const dyn MealExtractor as *const T)) }
    }
}

impl dyn MealPersister + 'static {
    pub fn downcast_ref<T: MealPersister + 'static>(&self) -> Option<&T> {
        unsafe { Some(&*(self as *const dyn MealPersister as *const T)) }
    }
}

#[derive(new, Debug, Clone, Default)]
pub struct MockCartPersister {
    pub cart: Option<Cart>,
}

impl MockCartPersister {
    pub fn verify_invoked(
        &self,
        cart: Option<&Cart>,
        cart_id: Option<&CartId>,
        meal_id: Option<&MealId>,
        customer_id: Option<&CustomerId>,
    ) {
        let self_cart = &self.cart.clone().unwrap();
        if cart.is_some() {
            assert_eq!(self_cart, cart.unwrap());
        }
        if cart_id.is_some() {
            assert_eq!(&self_cart.entity_param.id, cart_id.unwrap());
        }
        if meal_id.is_some() {
            assert_eq!(
                &self_cart.meals,
                &HashMap::from([(*meal_id.unwrap(), Count::one())])
            );
        }
        if customer_id.is_some() {
            assert_eq!(&self_cart.for_customer, customer_id.unwrap());
        }
    }

    pub fn verify_empty(&self) {
        assert!(&self.cart.is_none())
    }
}

impl CartPersister for MockCartPersister {
    fn save(&mut self, cart: Cart) {
        self.cart = Some(cart);
    }
}

#[derive(new, Clone, PartialEq, Debug, Default)]
pub struct MockCartRemover {
    pub id: Option<CartId>,
}

impl MockCartRemover {
    pub fn verify_invoked(&self, cart_id: CartId) {
        assert_eq!(self.id.unwrap(), cart_id)
    }

    pub fn verify_empty(&self) {
        assert!(&self.id.is_none())
    }
}

impl CartRemover for MockCartRemover {
    fn delete_cart(&mut self, cart: Cart) {
        self.id = Some(cart.entity_param.id);
    }
}

#[derive(new, Clone, PartialEq, Debug, Default)]
pub struct MockCartExtractor {
    pub cart: Option<Cart>,
    pub for_customer: Option<CustomerId>,
}

impl CartExtractor for MockCartExtractor {
    fn get_cart(&mut self, for_customer: CustomerId) -> Option<Cart> {
        self.for_customer = Some(for_customer);
        self.cart.as_ref().cloned()
    }
}

impl MockCartExtractor {
    pub fn verify_invoked(&self, for_customer: Option<CustomerId>) {
        assert_eq!(self.for_customer.unwrap(), for_customer.unwrap())
    }

    pub fn verify_empty(&self) {
        assert!(&self.cart.is_none())
    }
}

#[derive(new, Clone, PartialEq, Debug, Default)]
pub struct MockShopOrderExtractor {
    pub order: Option<ShopOrder>,
    pub id: Option<ShopOrderId>,
    pub for_customer: Option<CustomerId>,
    pub all: bool,
}

impl ShopOrderExtractor for MockShopOrderExtractor {
    fn get_by_id(&mut self, order_id: ShopOrderId) -> Option<ShopOrder> {
        self.id = Some(order_id);
        if self.order.clone().unwrap().entity_params.id == self.id.unwrap() {
            self.order.as_ref().cloned()
        } else {
            None
        }
    }

    fn get_last_order(&mut self, for_customer: CustomerId) -> Option<ShopOrder> {
        self.for_customer = Some(for_customer);
        if self.order.is_some() && self.order.clone().unwrap().for_customer == for_customer {
            self.order.as_ref().cloned()
        } else {
            None
        }
    }

    fn get_all(&mut self, _start_id: ShopOrderId, _limit: i32) -> Vec<ShopOrder> {
        self.all = true;
        if self.order.is_some() {
            vec![self.order.clone().unwrap()]
        } else {
            vec![]
        }
    }
}

impl MockShopOrderExtractor {
    pub fn verify_invoked_get_by_id(&self, id: ShopOrderId) {
        assert_eq!(self.id, Some(id));
        assert!(!self.all);
        assert!(self.for_customer.is_none());
    }

    pub fn verify_invoked_get_last_order(&self, for_customer: CustomerId) {
        assert_eq!(self.for_customer, Some(for_customer));
        assert!(!self.all);
        assert!(self.id.is_none());
    }

    pub fn verify_invoked_get_all(&self) {
        assert!(self.all);
        assert!(self.id.is_none());
        assert!(self.for_customer.is_none());
    }

    pub fn verify_empty(&self) {
        assert!(!self.all);
        assert!(self.id.is_none());
        assert!(self.for_customer.is_none());
    }
}
