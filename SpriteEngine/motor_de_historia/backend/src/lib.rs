//! # SoulForge v2.0
//! 
//! Sistema de generación procedural de personajes con profundidad psicológica
//! y relaciones emergentes.
//!
//! ## Características Principales
//! 
//! - **7 Capas del Alma**: Cada personaje tiene profundidad psicológica real
//! - **Química Relacional**: Las relaciones emergen de compatibilidades/conflictos psicológicos
//! - **Constelaciones**: Grupos de personajes interconectados con tensiones narrativas
//! - **Familias Generacionales**: Herencia de traumas y patrones
//! - **API REST**: Servidor para integración con otras herramientas

pub mod core;
pub mod relaciones;
pub mod constelacion;
pub mod exportadores;
pub mod api;
pub mod souls; // Registry of Souls System
pub mod cartographer;

// Re-exports principales
pub use core::{Alma, SoulForge, ParametrosGeneracion, Mundo, Rol, TonoMoral};
pub use constelacion::{Constelacion, ConstelacionBuilder};
pub use relaciones::{Quimica, TipoRelacion, Vinculo};
