# Proyecto 2 — Diorama con raytracing

Base del raytracer escrita solo con la biblioteca estándar de Rust. El primer avance muestra una pista y un entorno desértico muy sencillo, con una silueta roja de bloques en salto. No pretende ser todavía el modelo final del auto.

Ejecuta `cargo run` para abrir el boceto en una ventana. La imagen se calcula una sola vez y permanece estática; cierra la ventana con `Esc` o con el botón de cerrar.

La base ya incluye cámara con perspectiva, rayos, cubos con UV, texturas procedurales, materiales independientes (albedo, especular, reflectividad, transparencia e índice de refracción), iluminación Phong, sombras, reflexión, refracción y skybox procedural.
