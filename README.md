# Proyecto 2 — Diorama con raytracing

Base del raytracer escrita en Rust, con una ventana de visualización. El diorama recrea un circuito NASCAR completo visto desde arriba: óvalo, pit lane, boxes, paddock, tribunas, público, torre de resultados y pantalla gigante. Rayo McQueen y dos rivales estilizados —azul y verde— aparecen junto a la meta sin quitar protagonismo al estadio. Toda la geometría se genera por código; no se cargan modelos 3D.

Ejecuta `cargo run --release` para abrir el diorama a 960×600. Usa las flechas para orbitar, la rueda para acercar o alejar, `WASD` para desplazar el punto de inspección, `Shift` para moverte más rápido, `M` para enfocar la meta y el auto, `R` para recuperar la vista general y `F11` para alternar pantalla completa.

Incluye cinco materiales (asfalto, pintura roja, llanta, cristal y entorno), cada uno con textura procedimental y parámetros de iluminación. El cristal aporta transparencia, refracción y reflexión; la pintura del auto también es reflectiva. Incluye iluminación Phong, sombras y skybox procedural.

La escena cubre la rúbrica con cámara orbital y zoom, cinco materiales como máximo, texturas UV procedimentales, parámetros de albedo/especular/transparencia/reflectividad, cristal refractivo, pintura y asfalto reflectivos, múltiples luces y skybox.

## Verificación de la rúbrica

| Criterio | Implementación |
| --- | --- |
| Complejidad y atractivo | Óvalo, gradas de siete niveles, público, pits, paddock, torre, pantalla, luces y tres autos |
| Rotación y zoom | Flechas, rueda del mouse, WASD, Shift, vistas rápidas `M` y `R` |
| Cinco materiales | Asfalto, césped, pintura de carrera, material oscuro y cristal |
| Textura y parámetros propios | Checker, franjas, paleta de pintura y color sólido; cada material conserva albedo, especular, transparencia y reflectividad |
| Refracción | Cabinas, pantalla, luces y barreras usan cristal con IOR 1.5 |
| Reflexión | Pintura de los autos, asfalto y cristal trazan rayos reflejados |
| Skybox | Gradiente procedural entre horizonte y cenit |

Para obtener el mejor rendimiento usa `cargo run --release`. El render se divide automáticamente entre los núcleos disponibles del procesador; la primera compilación en modo release tarda más, pero la ejecución es considerablemente más rápida.
