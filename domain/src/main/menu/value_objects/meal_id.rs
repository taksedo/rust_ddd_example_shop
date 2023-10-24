use crate::main::menu::meal::MealError;
use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash)]
#[non_exhaustive]
pub struct MealId {
    pub value: i64,
}

impl MealId {
    pub fn to_i64(self) -> i64 {
        self.value.to_i64().unwrap()
    }

    pub fn from(value: i64) -> Result<MealId, MealError> {
        match value {
            x if x > 0 && x < i64::MAX => Ok(MealId { value }),
            _ => Err(MealError::IdGenerationError),
        }
    }
}

impl TryFrom<i64> for MealId {
    type Error = MealError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            x if x > 0 && x < i64::MAX => Ok(MealId { value }),
            _ => Err(MealError::IdGenerationError),
        }
    }
}
pub trait MealIdGenerator: Debug + Send {
    fn generate(&mut self) -> MealId;
}
