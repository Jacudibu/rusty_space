use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use common::types::ship_tasks::AwaitingSignal;

impl TaskComponent for ShipTask<AwaitingSignal> {
    fn can_be_aborted() -> bool {
        true
    }
}
