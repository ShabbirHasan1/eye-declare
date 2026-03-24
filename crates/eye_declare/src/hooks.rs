use std::marker::PhantomData;
use std::time::{Duration, Instant};

use crate::node::{Effect, EffectKind, TypedEffectHandler};

/// Effect collector for declarative lifecycle management.
///
/// Components receive a `Hooks` instance in their `lifecycle()` method
/// and use it to declare effects (intervals, mount/unmount handlers).
/// The framework clears old effects and applies the new set after
/// every build and update.
///
/// ```ignore
/// fn lifecycle(&self, hooks: &mut Hooks<MyState>, state: &MyState) {
///     if state.animating {
///         hooks.use_interval(Duration::from_millis(80), |s| s.advance_frame());
///     }
///     hooks.use_unmount(|s| s.cleanup());
/// }
/// ```
pub struct Hooks<S: 'static> {
    effects: Vec<Effect>,
    autofocus: bool,
    _marker: PhantomData<S>,
}

impl<S: Send + Sync + 'static> Hooks<S> {
    pub(crate) fn new() -> Self {
        Self {
            effects: Vec::new(),
            autofocus: false,
            _marker: PhantomData,
        }
    }

    /// Register a periodic interval effect.
    pub fn use_interval(
        &mut self,
        interval: Duration,
        handler: impl Fn(&mut S) + Send + Sync + 'static,
    ) {
        self.effects.push(Effect {
            handler: Box::new(TypedEffectHandler {
                handler: Box::new(handler),
            }),
            kind: EffectKind::Interval {
                interval,
                last_tick: Instant::now(),
            },
        });
    }

    /// Register a mount effect (fires once after the node is built).
    pub fn use_mount(&mut self, handler: impl Fn(&mut S) + Send + Sync + 'static) {
        self.effects.push(Effect {
            handler: Box::new(TypedEffectHandler {
                handler: Box::new(handler),
            }),
            kind: EffectKind::OnMount,
        });
    }

    /// Register an unmount effect (fires when the node is tombstoned).
    pub fn use_unmount(&mut self, handler: impl Fn(&mut S) + Send + Sync + 'static) {
        self.effects.push(Effect {
            handler: Box::new(TypedEffectHandler {
                handler: Box::new(handler),
            }),
            kind: EffectKind::OnUnmount,
        });
    }

    /// Request focus when this node mounts.
    ///
    /// If multiple nodes mount with autofocus in the same rebuild,
    /// the last one wins.
    pub fn use_autofocus(&mut self) {
        self.autofocus = true;
    }

    /// Whether autofocus was requested.
    pub(crate) fn autofocus(&self) -> bool {
        self.autofocus
    }

    /// Consume the hooks and return collected effects.
    pub(crate) fn into_effects(self) -> Vec<Effect> {
        self.effects
    }
}
