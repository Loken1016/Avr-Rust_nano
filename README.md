# Rust embebido para Arduino

## Aclaraciones antes de usar:

Estos programas fueron usados en un arduino nano y fueron totalmente funcionales hasta la fecha presente 03 de diciembre y han sido creados desde avr-hal-template, no estoy seguro si al crear el directorio y seleccionar el target de nano impida que mis programas funcionen en otros dispositivos como Arduino Uno por lo que si usted posee el antes mencionado y le da error puede crear un archivo desde avr-hal-template y copiar el codigo he intentar subir el codigo de nuevo.

## Pre requisitos y recomendaciones

Primero que nada recomiendo tener un Arduino Nano con el micro controlador ATMega328p que es la placa que yo uso por lo que es menos probable cualquier tipo de error.

Luego debes tener Rust instalado y luego instalar cargo-generate con el siguiente codigo ```cargo install cargo-generate``` para poder usar avr-hal-template con ```cargo generate --git https://github.com/Rahix/avr-hal-template.git```.

Tambien como es mi caso yo uso las herramientas de Avr para flashear mi Arduino, si tambien las quieres usar y utilizas Linux Arch puede ejecutar ```sudo pacman -S avr-gcc avr-binutils avr-libc avrdude``` o ver como se instala ```avr-gcc```, ```avr-binutils```, ```avrlibc``` y ```avrdude```.



## Como usar?

para usar los programas escritos en este repo tienes que copiar este repo o en su defecto descargarlo e ingresar a la carpeta llamada correct y al directorio del programa que quieres ejecutar y hacer un ```cargo build --release```.

luego del ejecutar el cargo build tendras que cambiar el nombre pwm1 por el nombre del programa q quiere subir a su arduino y ejecutar el siguiente comando que se encargara de pasar el archivo ```.elf``` a un archivo flasheable al arduino, osea un ```.hex```.

```avr-objcopy -O ihex -R .eeprom target/avr-none/release/pwm1.elf target/pwm1.hex```

Ya una vez el archivo este en formato ```.hex```  debe prestar atención a los ciertos factores del siguiente comando.

```avrdude -p atmega328p -c arduino -P /dev/ttyUSB0 -b 115200 -U flash:w:target/pwm2.hex```

En este
- Usamos ```avrdude``` para flashear el codigo
- ```-p atmega328p``` indica que el microcontrolador es un ATMega328p
- ```-c arduino``` a terminos simples indica que vamos a pasarle codigo al bootloader de Arduino
- ```-P /dev/ttyUSB0``` indica el puerto USB por el cual el computador se va a comunicar con el Arduino, a veces cambia ```USB0``` por ```ACM0```
- ```-b 115200``` indica la velocidad en bauds que se va a establecer la comunicacion entre el computador y el Arduino.
- ```U flash:w:target/pwm2.hex``` indica una escritura del programa ```pwm2.hex``` en la memoria del Arduino.



