//! Building blockz.

/// A set of functions every component should implement.
#[crate::async_trait]
pub trait ComponentExt<C> {
    /// Initialize the component.
    async fn init();
    /// Start the component.
    async fn start();
    /// Stop the component.
    async fn stop();
    /// Deinitialize the component.
    async fn deinit();
}

/// A component.
pub enum Component<C>
where
    C: ComponentExt<C>
{
    Cold(ColdComponent<C>),
    Initializing(InitializingComponent<C>),
    Starting(StartingComponent<C>),
    Running(RunningComponent<C>),
    Stopping(StoppingComponent<C>),
    Deinitializing(DeinitializingComponent<C>),
    Failed(FailedComponent<C>),
}

/// A component that awaits to be initialized.
pub struct ColdComponent<C>
where
    C: ComponentExt<C>
{
    inner: C,
}

/// A component that can be initialized.
pub struct InitializingComponent<C>
where
    C: ComponentExt<C>
{
    inner: C,
}

/// An initialized component that can be started.
pub struct StartingComponent<C>
where
    C: ComponentExt<C>
{
    inner: C,
}

/// A started component.
pub struct RunningComponent<C>
where
    C: ComponentExt<C>
{
    inner: C,
}

/// A started component that can be stopped.
pub struct StoppingComponent<C>
where
    C: ComponentExt<C>
{
    inner: C,
}

/// A stopped component that can be deinitialized.
pub struct DeinitializingComponent<C>
where
    C: ComponentExt<C>
{
    inner: C,
}

/// A component that failed in another stage.
pub struct FailedComponent<C>
where
    C: ComponentExt<C>
{
    inner: C,
}
