//! Building blockz.

/// Trait that defines the behaviour of a function that uses an immutable singleton.
pub trait SingletonFn<'s, S, R>
where
    R: Send
{
    type Res: std::future::Future<Output = crate::Result<R>>;
    fn call_once(self, inner: &'s S) -> Self::Res;
}

impl<'s, S, R, F, FR> SingletonFn<'s, S, R> for F
where
    S: 's,
    F: FnOnce(&'s S) -> FR,
    FR: std::future::Future<Output = crate::Result<R>> + 's,
    R: Send,
{
    type Res = FR;
    fn call_once(self, inner: &'s S) -> Self::Res {
        self(inner)
    }
}

/// Trait that defines the behaviour of a function that uses a mutable singleton.
pub trait SingletonFnMut<'s, S, R>
where
    R: Send
{
    type Res: std::future::Future<Output = crate::Result<R>>;
    fn call_once(self, inner: &'s S) -> Self::Res;
}

impl<'s, S, R, F, FR> SingletonFnMut<'s, S, R> for F
where
    S: 's,
    F: FnMut(&'s S) -> FR,
    FR: std::future::Future<Output = crate::Result<R>> + 's,
    R: Send,
{
    type Res = FR;
    fn call_once(mut self, inner: &'s S) -> Self::Res {
        self(inner)
    }
}

/// Define a singleton.
///
/// This also provides a few convenience functions.
#[macro_export]
macro_rules! define_singleton {
    ($name: ident, $type: ty) => {
        static $name: uslib::once_cell::sync::OnceCell<uslib::tokio::sync::Mutex<$type>> =
            uslib::once_cell::sync::OnceCell::new();
        impl $type {
            /// Run an async function with an immutable receiver.
            pub async fn use_singleton<F, R>(clojure: F) -> uslib::Result<R>
            where
                F: for<'c> uslib::blockz::SingletonFn<'c, $type, R>,
                R: Send,
            {
                let inner = $name.get().unwrap().lock().await;
                let inner_deref: &$type = &*inner;
                clojure.call_once(inner_deref).await
            }

            /// Run an async function with an mutable receiver.
            pub async fn use_mut_singleton<F, R>(mut clojure: F) -> uslib::Result<R>
            where
                F: for<'c> uslib::blockz::SingletonFnMut<'c, $type, R>,
                R: Send,
            {
                let inner = $name.get().unwrap().lock().await;
                let inner_deref: &$type = &*inner;
                clojure.call_once(inner_deref).await
            }
        }
    };
}

/// A set of functions every component should implement.
#[crate::async_trait]
pub trait ComponentExt {
    type Inner;
    type Config: Sync;

    /// Initialize the component.
    async fn init(config: &Self::Config) -> crate::Result<Self::Inner>;
    /// Start the component.
    async fn start(&mut self, config: &Self::Config) -> crate::Result<()> {
        Ok(())
    }
    /// Stop the component.
    async fn stop(&mut self, config: &Self::Config) -> crate::Result<()> {
        Ok(())
    }
    /// Deinitialize the component.
    async fn deinit(&mut self, config: &Self::Config) -> crate::Result<()>;
}

/// A component.
pub enum Component<C>
where
    C: ComponentExt + 'static,
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
    C: ComponentExt,
{
    inner: C,
}

/// A component that can be initialized.
pub struct InitializingComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// An initialized component that can be started.
pub struct StartingComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// A started component.
pub struct RunningComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// A started component that can be stopped.
pub struct StoppingComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// A stopped component that can be deinitialized.
pub struct DeinitializingComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// A component that failed in another stage.
pub struct FailedComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}
