# Luis Alejandro Hernandez Marquez (241424)
# Graficas por computadora
# Prof. Pablo Koch
# Proyecto 2 — Inicio de la Copa Pistón

Diorama 3D inspirado en la carrera inicial de *Cars*. La escena reúne a Rayo McQueen (95), El Rey (43) y Chick Hicks (86) junto a la línea de meta de un circuito estilo NASCAR, con gradas, público, pits, torre de control y palcos acristalados.

El escenario se construye mediante cubos, elipsoides y superficies ovaladas generadas por código, sin importar modelos 3D. Un raytracer propio calcula iluminación, sombras, reflexión y refracción. La cámara permite recorrer el estadio y acercarse para inspeccionar los autos y materiales.

## Tecnologías

- **Rust, edición 2024:** geometría, texturas procedurales y motor de raytracing en CPU.
- **Cargo:** compilación y gestión del proyecto.
- **Raylib 5.5:** ventana, entrada de teclado y mouse, y presentación de la imagen calculada.
- **Biblioteca estándar de Rust:** procesamiento multihilo y exportación de imágenes BMP.

## Ejecución

Con Rust y Cargo instalados, abre una terminal en la carpeta del proyecto:

```powershell
cd Proyecto_2
cargo run --release
```

Si la terminal ya está dentro de `Proyecto_2`, ejecuta únicamente `cargo run --release`.

La ventana es redimensionable y comienza en 1280 × 800. Mientras se mueve la cámara se muestra una vista previa; al detenerse, el render se refina automáticamente. Se recomienda el modo `release` para obtener mejor rendimiento.

## Controles

| Tecla o acción | Función |
| --- | --- |
| Flechas / arrastrar con el botón izquierdo | Rotar e inclinar la cámara |
| W, A, S, D / arrastrar con el botón derecho | Desplazar el punto de inspección |
| Rueda del mouse | Acercar o alejar |
| Shift izquierdo + W, A, S, D | Desplazarse más rápido |
| 1 / M | Vista de los tres autos en la final |
| 2 | Vista frontal de la meta |
| 3 | Vista lateral |
| 4 / R | Vista general del estadio |
| 5 | Inspeccionar los cristales de los palcos |
| 6 | Vista superior de los autos y sus números |
| F11 | Alternar pantalla completa |
| Esc | Cerrar la aplicación |

Las vistas también se pueden seleccionar con los botones de la barra superior.

## Video de demostración

Enlace del video

https://youtu.be/7v3zM-GNhJo
