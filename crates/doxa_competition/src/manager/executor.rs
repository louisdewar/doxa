/* use doxa_core::lapin::executor::Executor;
use wasmer::Instance;

pub enum ExecutorError {}

pub struct Agent {
    instance: Instance,
}

pub struct ExecutionManager<E: Executor> {
    executor: E,
    agents: Vec<Agent>,
}

impl<E: Executor> ExecutionManager<E> {
    pub fn new(agents: Vec<Agent>, executor: E) -> Self {
        ExecutionManager { agents, executor }
    }
}

pub struct ExecutionContext<'a, E: Executor> {
    manager: &'a mut ExecutionManager<E>,
}

impl<'a, E: Executor> ExecutionContext<'a, E> {}
*/
