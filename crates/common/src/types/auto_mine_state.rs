#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum AutoMineState {
    Mining,
    Trading,
}

impl AutoMineState {
    pub fn flip_task_depending_on_inventory(
        &mut self,
        used_inventory_space: u32,
        remaining_space_for_target_item: u32,
    ) {
        if self == &AutoMineState::Mining && remaining_space_for_target_item == 0 {
            *self = AutoMineState::Trading;
        } else if self == &AutoMineState::Trading && used_inventory_space == 0 {
            *self = AutoMineState::Mining;
        }
    }
}
