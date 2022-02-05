# Dělení pozemku v chatové oblasti

Úloha je převzata z: https://cw.felk.cvut.cz/brute/data/ae/release/2021z_b4b33alg/alg_cz_2021z/evaluation/input.php?task=grounds

Zastupitelstvo obce se rozhodne odprodat velký nevyužitý pozemek v chatové oblasti. Přihlásí se několik zájemců, ti ale poptávají pozemky menší rozlohy. Pověřený člen představenstva Ing. Novák se snaží na základě jejich požadavků stanovit, jak pozemek mezi ně rozdělit, aby maximalizoval zisk pro obec. Každý zájemce má jasnou představu o rozměrech svého pozemku. Zároveň nikdo nechce, aby jeho pozemek přímo sousedil s jiným pozemkem. Cena, kterou jsou ochotni zájemci zaplatit, se odvíjí především od vzdálenosti k hospodě Na Růžku, která se nachází v blízkosti severozápadního cípu pozemku. Atraktivitu nabídky mírně ovlivňují i další skutečnosti. Ing. Novák zakreslí na mapu pozemku pravidelnou čtvercovou mřížku a do každého pole mřížky zapíše jeho cenu odvozenou podle popsaných faktorů. Ukazuje se, že rozměry poptávaných pozemků se u všech zájemců dají vyjádřit právě jako násobky délky pole mřížky. Zároveň zájemci budou zcela jistě spokojeni, když odstup od sousedních pozemků bude roven alespoň uvedené jednotkové délce. Poslední, co zbývá, je zakreslit do mřížky optimální rozdělení polí.

## Úloha

Je dána čtvercová mřížka rozměrů M × N, ve které je každému poli přiřazena nezáporná celočíselná hodnota. Platí, že v každém řádku mřížky zleva doprava a v každém sloupci mřížky ze shora dolů tvoří hodnoty polí nerostoucí posloupnost.

Dále je dán seznam obdélníkových tvarů požadovaných pozemků. Cílem je umístit podmnožinu těchto obdélníků na mřížku tak, aby každé dva obdélníky měly prázdný průnik (přičemž se nesmí dotýkat ani jejich strany, ani jejich vrcholy) a aby součet hodnot polí mřížky pokrytých obdélníky byl maximální. Znamená to, že nemusí (a možná ani nemůže) být uspokojena poptávka všech zájemců.

Pokud je do mřížky umístěn obdélník o rozměru K × L, pak zakrývá obdélníkovou podoblast sestávající z K × L polí mřížky (nemůže však zakrývat podoblast L × K polí, jestliže L ≠ K, rotace obdélníku o 90° není povolena).

![image](https://user-images.githubusercontent.com/45851215/152635875-1a555c9c-bed2-4fd3-9600-b96f8c225f20.png)


## Vstup

Na prvním řádku jsou celá kladná čísla M a N oddělená mezerou. Číslo M je počet řádků čtvercové mřížky velkého pozemku, číslo N je počet jejích sloupců. Následuje M vstupních řádků, které reprezentují hodnoty polí mřížky po řádcích (všech M řádků tedy reprezentuje matici M × N). Čísla na řádcích jsou od sebe oddělená jednou či více mezerami. Další vstupní řadek obsahuje číslo T, které reprezentuje počet typů obdélníkových tvarů pozemků. Následuje T řádků, kde každý řadek reprezentuje jeden typ tvaru pomocí tří čísel K, L a P, kde K je výška (počet řádků) obdélníkového tvaru, L je jeho šířka (počet sloupců) a P je počet požadovaných pozemků daného tvaru (několik zájemců o pozemky může mít o tvaru stejnou představu). Čísla K, L a P splňují 1 ≤ K ≤ M, 1 ≤ L ≤ N a 1 ≤ P ≤ 12.

Platí 1 ≤ M, N ≤ 22. Každé pole mřížky má hodnotu menší než 100.

## Výstup

Výstup obsahuje jeden řádek, na kterém je jedno číslo, které odpovídá maximálnímu součtu hodnot polí čtvercové mřížky pokrytých obdelníky.

### Příklad 1

#### Vstup
```
4 6
9 9 9 8 6 4
9 8 8 8 5 3
8 7 6 6 4 2
6 5 5 5 3 1
2
2 1 2
1 2 4
```

#### Výstup
```
64
```

*Data a řešení Příkladu 1 jsou znázorněna na Obrázku 1.*

### Příklad 2
#### Vstup
```
3 3
6 4 2
5 4 2
3 2 1
1
1 1 3
```

#### Výstup
```
11
```
