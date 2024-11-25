use std::any::Any;

// Trait für die Umwandlung in Any
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

// Generische Implementierung für alle Typen, die das Any-Interface unterstützen
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Zusatz: Erweiterung für Mut-Referenzen (falls nötig)
pub trait AsAnyMut {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAnyMut for T {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
