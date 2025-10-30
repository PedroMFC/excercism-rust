use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Convertir todo a minutos totales
        let total_minutes = hours * 60 + minutes;
        
        // Normalizar a minutos dentro de un día (1440 minutos = 24 horas)
        let normalized_minutes = ((total_minutes % 1440) + 1440) % 1440;
        
        Self {
            hours: normalized_minutes / 60,
            minutes: normalized_minutes % 60
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Convertir el reloj actual a minutos totales y sumar
        let total_minutes = self.hours * 60 + self.minutes + minutes;
        
        // Normalizar usando la misma lógica que new()
        let normalized_minutes = ((total_minutes % 1440) + 1440) % 1440;
        
        Self {
            hours: normalized_minutes / 60,
            minutes: normalized_minutes % 60
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}