# Proyecto 2 — Diorama con raytracing

Base del raytracer escrita en Rust, con una ventana de visualización. El diorama recrea el Motor Speedway de *Cars*: pista ovalada, isla verde, zona de pits, gradas, pantalla gigante, arco de meta y un Rayo McQueen estilizado con cubos y elipsoides.

Ejecuta `cargo run` para abrir el diorama. Usa las flechas para orbitar la cámara y la rueda del mouse para acercar o alejar. Cada cambio recalcula el raytracing; cierra con `Esc` o el botón de cerrar.

Incluye cinco materiales (asfalto, pintura roja, llanta, cristal y entorno), cada uno con textura procedimental y parámetros de iluminación. El cristal aporta transparencia, refracción y reflexión; la pintura del auto también es reflectiva. Incluye iluminación Phong, sombras y skybox procedural.

Para obtener el mejor rendimiento usa `cargo run --release`. El render se divide automáticamente entre los núcleos disponibles del procesador; la primera compilación en modo release tarda más, pero la ejecución es considerablemente más rápida.
