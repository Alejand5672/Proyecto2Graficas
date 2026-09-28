# Proyecto 2 — Diorama con raytracing

Base del raytracer escrita en Rust, con una ventana de visualización. El diorama recrea una salida de carrera inspirada en *Cars*: pista, desierto, arco de meta y un auto rojo formado por cubos.

Ejecuta `cargo run` para abrir el diorama. Usa las flechas para orbitar la cámara y la rueda del mouse para acercar o alejar. Cada cambio recalcula el raytracing; cierra con `Esc` o el botón de cerrar.

Incluye cinco materiales (asfalto, pintura roja, llanta, cristal y desierto), cada uno con textura procedimental y parámetros de iluminación. El cristal aporta transparencia, refracción y reflexión; la pintura del auto también es reflectiva. Incluye iluminación Phong, sombras y skybox procedural.
