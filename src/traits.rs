use std::any::Any;

// Definition des AsAny-Traits
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

// Implementierung des AsAny-Traits für alle Typen, die Any implementieren
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
