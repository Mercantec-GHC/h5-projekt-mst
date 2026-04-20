
# Case-beskrivelse

Kunden ønsker et Slope-agtigt spil, med en fysisk skateboard-agtig enhed, som spilleren bruger til at styre spillet.

Slope er et computerspil, hvor en kugle triller ned af en bane. Banen består af segmenter, som hver er tynde slisker med forskellige forhindringer. Spilleren ser kuglen bagfra i en 3D-projektion. Opgaven går ud på at styre kuglen i sideværts retning, for at blive på banen og undgå forhindringerne. For hvert segment, får spilleren et point. Spillet er uendligt, dvs. det handler om at få så mange point som muligt, før man falder ud af banen eller rammer en forhindring.

Den fysiske skateboard-enhed skal gøre det muligt for spilleren, at styre den sideværtse bevægelse i spillet ved at stå og vippe på et bræt.

# Krav-specifikation

## Målgruppe

Målgruppen er normale personer som enten i individuelle eller forsamlingsmæssige sammenhænge ønsker en sjov og immersiv spiloplevelse. Spillet skal være nemt at komme i gang med, men tillade konkurrence mellem spillere og progression.

## Krav

### Slope-agtigt spil

Løsningen skal indeholde et spil. Spillet skal være et Slope-agtigt spil, hvor man styrer et object ned af en bakke med forhindringer. Spilleren skal kunne styre objektet sideværts, med målet om at undgå forhindringerne.

Man skal kunne:
- **spille spillet på et PC-setup.** Et PC-setup består af som minimum af en skærm, mus, tastatur og en computer. Det antages, at kunden besider hardware produceret indenfor årtusindeskiftet.
- **se et spiller-objekt på skærmen**. Der skal vises et objekt på skærmen, som tydeligt repræsenterer det, som spilleren skal styre. Spiller-objektet skal til en hvis grad vise spillerens input.
- **løbende når man spiller.** Spiller er uendeligt, men for hvert segment i spillet, tjener spilleren et point. Points'ne lægges sammen og repræsenterer spillerens point-score.
- **opleve voksende udfordring løbende når man spiller.** Spillet er uendeligt, men sværhedsgraden skal stige løbende som spilleren spiller. Herved repræsenterer en større point-score ikke kun et længere spil, men også et sværre spil.
- **have en god spiloplevelse på standard hardware.** Spillet skal køre over 25 FPS på en standard bærbar computer.

### Fysisk skateboard-controller

Løsningen skal indeholde en fysisk skateboard-agtig controller, som spilleren kan bruge til at spille spillet. Den fysiske controller skal være et skateboard-agtigt bræt, hvor spilleren kan spille spillet, ved at stå og vippe på brættet.

Man skal kunne:
- **forbinde et bræt til et spil kørende på en PC.** Skateboard-brættets inputs skal reflekteres i spillet, så spilleren kan bruge det til at spille.
- **vippe på brættet som input til spillet.** Spilleren skal kunne stå på brættet og vippe det sideværts. Brættet skal opfange disse vip, og sende det som input til spillet.
- **have en god spiloplevelse.** Brættet skal sende inputs til spillet, så oplevelsen føles responsiv. Inputs må sendes med en maksimal forsinkelse på 1 sekund.



