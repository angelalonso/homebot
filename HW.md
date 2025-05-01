# Connections Raspi <-> L298N
                --1-|O O|--2-
                --3-|O O|--4-
                --5-|O O|--6-
                --7-|O O|--8-
                --9-|O.O|-10-
            IN1 -11-|O O|-12-
            IN2 -13-|O O|-14-
            ENA -15-|O O|-16- IN3
                -17-|O O|-18- IN4
                -19-|O.O|-20-
                -21-|O O|-22- ENB
                -23-|O O|-24-
                -25-|O O|-26-
                -27-|O O|-28-
                -29-|O.O|-30-
                -31-|O O|-32-
                -33-|O O|-34-
                -35-|O O|-36-
                -37-|O O|-38-
                -39-|O O|-40-

# GUIDES
## Raspberry Pi's 3 Model B GPIO

                  +3V3---1-|O O|--2--+5V
          (SDA)  GPIO2---3-|O O|--4--+5V
         (SCL1)  GPIO3---5-|O O|--6-- GND
    (GPIO_GLCK)  GPIO4---7-|O O|--8-----GPIO14 (TXD0)
                   GND --9-|O.O|-10-----GPIO15 (RXD0)
    (GPIO_GEN0) GPIO17--11-|O O|-12-----GPIO18 (GPIO_GEN1)
    (GPIO_GEN2) GPIO27--13-|O O|-14-- GND
    (GPIO_GEN3) GPIO22--15-|O O|-16-----GPIO23 (GPIO_GEN4)
                  +3V3--17-|O O|-18-----GPIO24 (GPIO_GEN5)
     (SPI_MOSI) GPIO10--19-|O.O|-20-- GND
     (SPI_MISO) GPIO9 --21-|O O|-22-----GPIO25 (GPIO_GEN6)
     (SPI_SCLK) GPIO11--23-|O O|-24-----GPIO8  (SPI_C0_N)
                   GND -25-|O O|-26-----GPIO7  (SPI_C1_N)
       (EEPROM) ID_SD---27-|O O|-28-----ID_SC Reserved for ID EEPROM
                GPIO5---29-|O.O|-30-- GND
                GPIO6---31-|O O|-32-----GPIO12
                GPIO13--33-|O O|-34-- GND
                GPIO19--35-|O O|-36-----GPIO16
                GPIO26--37-|O O|-38-----GPIO20
                   GND -39-|O O|-40-----GPIO21

## L298N GPIO
OUT1 -O                    O- OUT3
OUT2 -O                    O- OUT4
       O O O    O O O O O O
       | | |    | | | | | |
       1 G 5    E I I I I E
       2 N V    N N N N N N
       V D      A 1 2 3 4 B


