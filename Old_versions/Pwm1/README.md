Explicación del código

Este código está basado en una plantilla de Git. A continuación se explica su funcionamiento paso a paso:

Configuración inicial:

En Rust embebido no se usan librerías estándar ni una función main tradicional.

Por eso las dos primeras líneas (#![no_std] y #![no_main]) indican que no habrá librerías estándar ni un main convencional.

Dependencias:

Se usan las crates embedded_hal y arduino_hal, que proporcionan las funciones específicas para programar el Arduino.

Con esto Rust solo carga lo necesario para trabajar en un entorno embebido.

Función fade:

Define un bucle que ajusta el ciclo de trabajo PWM del LED.

Los parámetros led y delay permiten modificar el brillo y controlar el tiempo de espera.

La función no devuelve nada (-> !), lo que significa que se convierte en un bucle infinito.

Control de brillo:

Dentro de fade se usa un for que incrementa y decrementa el brillo del LED, creando un efecto de "respiración".

Timer y configuración del pin:

Se configura el timer0, que controla los pines 5 y 6 del Arduino.

El pin 5 se selecciona como salida PWM y se vincula al timer0.

Delay:

Finalmente, se crea un nuevo delay que se usa junto al LED dentro del bucle para controlar el encendido y apagado progresivo.
