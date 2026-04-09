# EOP

## Datatransmission over netværk

## Cyberangreb

## Ssh

## Man in the middle

## Kryptering

## Diffie-hellman

## RSA
RSA (Rivest–Shamir–Adleman) er en af de første algoritmer der bruger Diffie-Hellman metoden til datatransmission på en sikker måde. Den måde RSA fungere på er ved at man nemt kan finde produktet af 2 primtal. Det er derimod svært at finde hvilke 2 primtal der er skal til for at finde produktet. Her er et eksempel.

Vi siger at vi har 2 primtal e og q

e = 38183

p = 11731

Og nu vil vi gerne finde produktet n af disse 2 primtal

e * q = n

n = 11731 * 38183 = 447924773

Her kan vi finde ud af at n var 447924773. Lad os nu sige at vi har n i stedet og vi gerne vil finde ud af e og q i stedet.

n = 447924773

447924773 = e * q

Vi kan finde specifikke relationer mellem n, e og q

447924773 / q = e

447924773 / e = q

Men med 2 ubekendte og en ligning er det utroligt svært at finde de 2 ubekendte. Dette af hvad RSA algoritmen gør brug af. Vi kan derfor bruge q og e som private key og n og public key.

## ED25519

## Eliptiske kurver