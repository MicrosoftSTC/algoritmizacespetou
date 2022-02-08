# Algoritmizace s Péťou

[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/MicrosoftSTC/alogoritmizacespetou)

Projekt je momentálně ve vývoji. Chceš se zapojit? Napiš mi petr.kucera@studentstc.cz.

## Description

Cílem projektu je se zlepšit v algoritmizačních úlohách pomocí praktických úloh. Zadání jsou posbíraná z různých webů a univerzitních zadání.

## How it works

1. Forkni si repozitář.
2. Vyber si algoritmizační úlohu.
    - každá úloha obsahuje veřejná data, zadání úkolu
4. Vyřeš ji.
5. Máš otázku? Nenapadá tě řešení? Máš nápad na zlepšení, vytvoř issues a pojď diskutovat! 👈
6. Nahraj řešení, které se ti automaticky validuje pomocí GitHub Actions. (Aby řešení prošlo, musí vracet nejen správné výsledky, ale musí být i dostatečně rychlé.)

## Validátor

Při validaci jsou podporovány následující jazyky a jsou kompilovány s flagy:

| jazyk | commandy |
|-------|---------- |
| C++ | ```g++ -std=c++14 -pipe -Wall -O3 -c *.cpp``` ```g++ -std=c++14 -pipe -Wall -O3 -c *.cxx``` ```g++ -std=c++14 *.o -o main``` ```./main``` |
| C | ```gcc --std=c99 -pipe -Wall -O3 -c *.c``` ```g++ -std=c++14 *.o -o main``` ```./main``` |
