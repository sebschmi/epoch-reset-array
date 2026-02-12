use std::{
    fmt::{Debug, Display},
    iter,
    marker::PhantomData,
    mem,
};

use num_traits::{CheckedAdd, One, Zero};

#[cfg(test)]
mod tests;

pub trait EpochResetArrayIndex: TryFrom<usize> + TryInto<usize> + Display + Copy {}

impl<T: TryFrom<usize> + TryInto<usize> + Display + Copy> EpochResetArrayIndex for T {}

pub trait EpochResetArrayValue: Clone {}

impl<T: Clone> EpochResetArrayValue for T {}

pub trait EpochResetArrayCounter: Zero + One + CheckedAdd + Eq + Copy + Debug {}

impl<T: Zero + One + CheckedAdd + Eq + Copy + Debug> EpochResetArrayCounter for T {}

#[derive(Clone)]
pub struct EpochResetArray<Index, Value, EpochCounter> {
    array: Vec<EpochValue<Value, EpochCounter>>,
    reset_value: Value,
    epoch_counter: EpochCounter,
    phantom_data: PhantomData<Index>,
}

#[derive(Clone)]
struct EpochValue<Value, EpochCounter> {
    value: Value,
    epoch_counter: EpochCounter,
}

impl<Index: EpochResetArrayIndex, Value: EpochResetArrayValue, EpochCounter: EpochResetArrayCounter>
    EpochResetArray<Index, Value, EpochCounter>
{
    pub fn new(reset_value: Value, len: Index) -> Self {
        Self {
            array: Vec::from_iter(
                iter::repeat_with(|| EpochValue::new(reset_value.clone(), EpochCounter::zero()))
                    .take(index_to_usize(len)),
            ),
            reset_value,
            epoch_counter: EpochCounter::zero(),
            phantom_data: PhantomData,
        }
    }

    pub fn len_usize(&self) -> usize {
        self.array.len()
    }

    pub fn len_index(&self) -> Index {
        usize_to_index(self.array.len())
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn get(&self, index: Index) -> &Value {
        let epoch_value = &self.array[index_to_usize(index)];
        if epoch_value.epoch_counter == self.epoch_counter {
            &epoch_value.value
        } else {
            &self.reset_value
        }
    }

    pub fn get_mut(&mut self, index: Index) -> &mut Value {
        let epoch_value = &mut self.array[index_to_usize(index)];
        if epoch_value.epoch_counter != self.epoch_counter {
            *epoch_value = EpochValue::new(self.reset_value.clone(), self.epoch_counter);
        }
        &mut epoch_value.value
    }

    pub fn set(&mut self, index: Index, value: Value) -> Option<Value> {
        let epoch_value = &mut self.array[index_to_usize(index)];
        if epoch_value.epoch_counter == self.epoch_counter {
            Some(mem::replace(&mut epoch_value.value, value))
        } else {
            *epoch_value = EpochValue::new(value, self.epoch_counter);
            None
        }
    }

    pub fn reset(&mut self) {
        self.epoch_counter = self
            .epoch_counter
            .checked_add(&EpochCounter::one())
            .unwrap_or_else(|| {
                // Actually reset values when counter overflows.
                self.array.iter_mut().for_each(|epoch_value| {
                    *epoch_value = EpochValue::new(self.reset_value.clone(), EpochCounter::zero());
                });

                EpochCounter::zero()
            });
    }
}

impl<Value, EpochCounter> EpochValue<Value, EpochCounter> {
    fn new(value: Value, epoch_counter: EpochCounter) -> Self {
        Self {
            value,
            epoch_counter,
        }
    }
}

fn index_to_usize<Index: EpochResetArrayIndex>(index: Index) -> usize {
    index
        .try_into()
        .unwrap_or_else(|_| panic!("Index {index} cannot be converted to usize"))
}

fn usize_to_index<Index: EpochResetArrayIndex>(index: usize) -> Index {
    index
        .try_into()
        .unwrap_or_else(|_| panic!("Usize {index} cannot be converted to index type"))
}
