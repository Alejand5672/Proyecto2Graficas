# Proyecto 2 — Final de la Copa Pistón

Diorama estilizado con raytracing CPU en Rust. Se conserva Raylib 5.5 para ventana, controles y presentación de la imagen. La geometría (cubos, elipsoides y anillos elípticos), las texturas, las sombras y los reflejos se calculan en el proyecto. No hay modelos 3D importados.

## Ejecutar

```powershell
cd C:\Graficas\Proyecto_2
cargo run --release
```

Ventana inicial 1280×800, redimensionable. La imagen se adapta al tamaño y a su relación de aspecto. Durante el movimiento se renderiza a 320 píxeles de ancho; al detenerse se calcula a 960, con cuatro muestras por píxel y cuatro rebotes. El render ocurre en segundo plano: la ventana sigue atendiendo eventos. No se promete raytracing a 60 FPS; la actualización final puede tardar varios segundos según la máquina.

## Controles

- 1 / M: composición de la final; 2: frontal; 3: lateral; 4 / R: estadio.
- Botones superiores: selección de las mismas cuatro vistas.
- Arrastre izquierdo o flechas: órbita; derecho / WASD: desplazamiento.
- Rueda: zoom; Shift: desplazamiento rápido; F11: pantalla completa.
- Touch: arrastre de un dedo y pellizco de dos dedos cuando el backend proporciona eventos. No validado con hardware táctil.
- Límites de distancia e inclinación, y elevación automática si el observador queda dentro de un bloque.

## Cambios importantes

- Las normales de los cubos escalados se calculan según sus dimensiones: arregla iluminación y sombras de suelo y edificios.
- Las doce ruedas se apoyan en Y=-1.055 independientemente de la escala del auto.
- Se conservan las carrocerías elipsoidales; se añaden ojos opacos, pupilas, rines, números 95/43/86, faros y alerones diferenciados.
- Concreto y metal sustituyen el uso incorrecto del vidrio en edificios, muros y gradas.
- Tribunas con soporte hasta el suelo y público procedural; sin miles de objetos individuales.
- Meta de dos filas, vallas metálicas, panel Copa Pistón y terreno extendido.
- Vidrio: normal orientada al cruzar la superficie y reflexión interna total.
- Cielo procedural envolvente con nubes y resplandor solar alineado con la luz principal.
- Sombras con dos muestras de luz, ambiente hemisférico y tone mapping moderado.
- BVH para descartar grupos de objetos, procesamiento por hilos y presentación con una textura GPU.

## Módulos

| Archivo | Función |
| --- | --- |
| src/scene.rs | Conserva y ajusta estadio, pista, materiales y luces |
| src/cars.rs | Detalles de los tres vehículos y letras geométricas |
| src/accel.rs | Aceleración espacial de las intersecciones |
| src/renderer.rs | Raytracing, sombras, reflexión, refracción y antialiasing |
| src/display.rs / src/views.rs | Ventana, render asíncrono, controles y vistas |
| src/cube.rs / src/ellipsoid.rs / src/oval.rs | Geometría analítica y coordenadas UV |
| src/texture.rs / src/color.rs | Texturas, color y exportación BMP |
| src/regression.rs | Pruebas de normales, contacto, BVH y reflexión interna |

## Rúbrica: evidencia y pendientes

| Criterio | Estado verificable |
| --- | --- |
| Complejidad / atractivo | Circuito, pits, torre, gradas, público y tres autos. La valoración visual es subjetiva; el acabado sigue siendo estilizado. |
| Rotación y zoom | Implementados; vistas frontal, lateral, general y final. |
| Materiales | Asfalto, césped, pintura, caucho, vidrio, concreto, metal, señalización y público; variantes de pintura por auto. Todos declaran parámetros ópticos. |
| Texturas propias | Procedurales; caucho y señalización usan color constante. Si se requiere textura detallada en cada material puntuable, esos dos no deben contarse como texturas elaboradas. |
| Reflexión | Rayos recursivos sobre pintura, metal y vidrio. |
| Refracción | Cabinas cerradas con IOR 1.5. |
| Skybox | Entorno procedural direccional; no es un cubemap de seis imágenes. Confirmar si la evaluación exige ese formato. |
| Sin modelos externos | Cumplido: toda la geometría está programada. |
| Sin bibliotecas externas | Pendiente: Raylib ya era una dependencia y se conserva por el requisito de mantener el motor/framework. |
| Video en README / entrega GitHub | Pendiente de grabar y publicar. |

La nueva solicitud pide al menos cinco materiales y permite adicionales. Esta versión usa más de cinco familias para separar correctamente vidrio, metal, caucho y concreto; la rúbrica adjunta limita la puntuación por materiales a cinco. Si el docente impone un límite absoluto de cinco, será necesaria una versión reducida.

## Verificación reproducible

```powershell
cargo test
cargo run --release -- --render 0 preview-final.bmp
cargo run --release -- --render 1 preview-front.bmp
cargo run --release -- --render 2 preview-side.bmp
cargo run --release -- --render 3 preview-stadium.bmp
```

Las cuatro pruebas comprueban normales de bloques escalados, contacto de las doce ruedas, equivalencia BVH/búsqueda lineal y reflexión interna total. Las exportaciones permiten revisar las cámaras sin abrir una ventana. Las cifras de tiempo impresas incluyen solo el render de cada vista.
