
    fn madurez(rng: &mut impl Rng, nombre: &str, edad: u32) -> String {
        let opciones = vec![
            format!("Pasados los años de juventud, {} entró en una etapa de consolidación. Las dudas existenciales dieron paso a preguntas más prácticas: ¿Qué estoy construyendo realmente?", nombre),
            format!("La madurez llegó a {} no como una respuesta, sino como una calma. Dejó de luchar contra corrientes que no podía cambiar y empezó a navegar las que sí podía.", nombre),
            format!("A los {} años, la perspectiva cambió. Lo que antes parecía vital se volvió trivial, y lo trivial, sagrado.", edad - 10),
        ];
        opciones.choose(rng).unwrap().clone()
    }

    fn legado(rng: &mut impl Rng, nombre: &str, _edad: u32) -> String { // Underscore para variable no usada si no la uso
        let opciones = vec![
            format!("Mirando atrás, {} empezó a pensar no en lo que ganaría, sino en lo que dejaría. El legado se convirtió en una obsesión silenciosa.", nombre),
            format!("{} ha visto partir a muchos. Amigos, rivales. Eso le ha dado una soledad particular, una que se lleva con dignidad.", nombre),
            format!("Ya no queda nada que demostrar. Solo queda vivir acorde a la propia verdad, cueste lo que cueste.", nombre),
        ];
        opciones.choose(rng).unwrap().clone()
    }
    
    fn crepusculo(rng: &mut impl Rng, nombre: &str, _edad: u32) -> String {
        let opciones = vec![
            format!("En el invierno de su vida, {} ha encontrado una claridad que los jóvenes confundirían con cinismo. No lo es. Es paz.", nombre),
            format!("El tiempo se ha vuelto circular para {}. El pasado y el presente se mezclan. Los recuerdos son tan vívidos como el ahora.", nombre),
            format!("{} espera el final sin miedo. Ha visto suficiente mundo para saber que todo ciclo debe cerrarse.", nombre),
        ];
        opciones.choose(rng).unwrap().clone()
    }
