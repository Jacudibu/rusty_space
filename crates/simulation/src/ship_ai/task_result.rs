/// Simple enum to tell how a task went. More complex tasks might opt to have their own types.
pub enum TaskResult {
    Ongoing,
    Finished,

    /// Aborting is a special case: Something went wrong, it was unexpected, and unless the user triggered it, it should probably be logged.
    /// Maybe the target entity was suddenly missing (destroyed), certain components could not be found, or due to travel settings, the target has become unreachable.
    ///
    /// Whatever it may be, this task now requires some special handling to decide what to do next.
    Aborted,
}
