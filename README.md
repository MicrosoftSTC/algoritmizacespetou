# Algoritmizace s Péťou

Projekt je momentálně ve vývoji. Chceš se zapojit? Napiš mi petr.kucera@studentstc.cz.

## Description

Cílem projektu je se zlepšit v algoritmizačních úlohách pomocí praktických úloh.

## How it works

1. Forkni si repozitář.
2. Vyber si alogrimtizační úlohu.
    - každá úloha obsahuje veřejná data, zadání úkolu
4. Vyřeš ji.
5. Nahraj řešení, které se ti automaticky zvaliduje pomocí GitHub Actions. (Aby řešení prošlo, musí vracet nejen správé výsledky, ale musí být i dostatečně rychlé.)

## Validátor

Při validaci jsou podporovány následující jazyky a jsou kompilovýny s flagy:

| jazyk | commandy |
|-------|---------- |
| C a C++ | ```g++ -std=c++14 -pipe -Wall -O3 -c *.cpp``` ```g++ -std=c++14 -pipe -Wall -O3 -c *.cxx``` ```gcc --std=c99 -pipe -Wall -O3 -c *.c``` ```g++ -std=c++14 *.o -o main``` ```./main``` |
