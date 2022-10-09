use crate::ai::neural::Neural;
use crate::field::{Cell, OtherBot};
use rand::Rng;

pub enum Action {
    Move,
    Reproduction,
    Attack,
    Heal,
}

#[derive(Clone, Debug)]
pub struct Bot {
    pub brain: Neural,
    pub energy: isize,
    pub id: u8,
    pub color: (u8, u8, u8),
}

impl Bot {
    pub fn new() -> Self {
        let input_layers_num = 5;
        let layers_num = 15;
        let layers_size = 5;
        let output_layers_num = 4;

        Self {
            brain: Neural::new(input_layers_num, layers_num, layers_size, output_layers_num),
            energy: 10,
            id: 1,
            color: (0, 0, 0),
        }
    }

    pub fn step(&self, around: Vec<(i8, Option<OtherBot>)>) -> (u8, Action) {
        let mut result_action = (0.0, 0, Action::Heal);

        for (direction, (emptiness, other)) in around.iter().enumerate() {
            if emptiness == &-1 {
                continue;
            } else if emptiness == &0 {
                let result = self.brain.execute(vec![
                    self.id as f64 / 10.0,
                    self.energy as f64 / 5.0,
                    direction.clone() as f64 / 8.0,
                    0.0,
                    0.0,
                ]);
                let moving = result.get(0).unwrap();
                if moving > &result_action.0 {
                    result_action = (moving.clone(), direction.clone(), Action::Move);
                }
                let reproduction = result.get(1).unwrap();
                if reproduction > &result_action.0 {
                    result_action = (
                        reproduction.clone(),
                        direction.clone(),
                        Action::Reproduction,
                    );
                }
                let heal = result.get(3).unwrap();
                if heal > &result_action.0 {
                    result_action = (heal.clone(), direction.clone(), Action::Heal);
                }
            } else if emptiness == &1 {
                if let Some(other) = other {
                    let result = self.brain.execute(vec![
                        self.id as f64 / 10.0,
                        self.energy as f64 / 5.0,
                        direction.clone() as f64 / 8.0,
                        other.id as f64 / 10.0,
                        other.energy as f64 / 5.0,
                    ]);

                    let attack = result.get(2).unwrap();
                    if attack > &result_action.0 {
                        result_action = (attack.clone(), direction.clone(), Action::Attack);
                    }
                    let heal = result.get(3).unwrap();
                    if heal > &result_action.0 {
                        result_action = (heal.clone(), direction.clone(), Action::Heal);
                    }
                }
            }
        }
        return (result_action.1 as u8, result_action.2);
    }

    pub fn mutate(&self) -> Self {
        let mut new_me = self.clone();
        if rand::thread_rng().gen_range(0..100) < 10 {
            new_me.energy = 10;
        }
        if rand::thread_rng().gen_range(0..100) < 2 {
            new_me.brain.mutate();
            new_me.id = new_me.id + 1;
            if new_me.id > 9 {
                new_me.id = 0;
            }
            new_me.color = (
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
            );
        }
        new_me
    }
}
