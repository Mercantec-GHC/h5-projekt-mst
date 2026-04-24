
# Processrapport

M. Kongsted, S. F. Jakobsen &lt;sfja2004@gmail.com&gt;, T. P. Hollebeek

Som forberedelse til forløbet diskuterede vi diverse ambitioner og forventninger. Vores mål var at opnå en forventningsafstemning, for hvor bredt omfang projektet skulle havde. I gruppen er vi i den situation, at det faglige niveau blandt gruppmedlemmerne variere betydeligt. Dette kræver en større indsats for koordinering og forventningsafstemning, så den ene ende af gruppen får mulighed for faglig udfoldelse, samtidig med at den anden ende kan være med. Vi valgte aktivt, at forebygge eventuelle konflikter, ved at tage nogle af disse diskussioner før forløbet.

## Idegenerering

*20-3-2026*

Vi påbegyndte forløbet med en brainstorm over, hvilket projekt vi ville arbejde med. Vi havde udfordringer med at komme på ideer, så vi gik metodisk til værks. Vores prioritet i starten var at skrive så mange ideer på et whiteboard som muligt. Vores metode gik ud på, at vi tænkte på en type sensor, og så tænkte vi på projekter, hvor man kunne inddrage sensoren.

Vi fik skrevet en håndful ideer på tavlen. Disse ideer blev omdiskuteret internt i gruppen. En af gruppemedlemmerne var ikke tilstede, men fik en mundtlig gennemgang af de brainstormede ideer.

Dagen efter var hele gruppen samlet. Her blev ideerne diskuteret og evalueret igen. Med baggrund i erfaring og estimater, blev nogle ideer valgt fra, og andre blev uddybet.

Vi endte ud i 2 ideer. Den ene en dart-tracker, som via. kameraer skulle kunne tracke score på en dartplade. Den anden var et Slope-agtigt spil, hvor man styrer med et skateboard. Den første ide blev valgt fra efter en faglig vurdering om, at projektet ikke var kompatibel med vores forventningsafstemning. Den anden ide var derved blevet valgt.

Med det samme efter at havde valgt ideen med Slope-spil og skateboard, gik vi i gang med at uddybe ideen og tænke på en mulig implementering. Vi vidste, ud fra kravene til projektet, at vi skulle inddrage forskellige dele i vores løsning. Dette galdt eksempelvis en sensor og en IoT-device. Vi gik i gang med at lave research på mulig hardware. Vi valgte at bruge et accelerometer/gyroskop-komponent til vores skateboard. Af en betragtning af, hvilke lignende projekter vi kunne finde på internettet, vores erfaring og hvad vi kunne tænke os, at lære om, valgte vi en ESP32 SoC og en MPU6050 sensormodul.

Vi bestilte hardware'en med det samme. Ideen var, at jo hurtigere vi fik mulighed for at arbejde med hardware'en, desto hurtigere ville vi finde ud af, om det passede vores behov. I mellemtiden kiggede vi efter hardware med lignende funktionalitet. Skolen havde et Arduino OPLA kit tilrådighed. Her kunne vi lave et eksperiment med OPLA-hattens accelerometer/gyroskob og den Arduino MKR 1010 WiFi, som følger med. Med dette setup og Arduino's manual, lavede vi et program, som aflæste målingsværdierne og plottede det på en visuel graf.

![](./acceleration_graph.png)
MPU'ens accelerations- og vinkelaccelerationsmålinger plottet over tid på en graf.

*23-3-2026*

Efter weekenden kom vores hardware. Omgående gik vi i gang med at lave et setup til at påvise, at hardware'en passede til vores behov. I første omgang krævede det, at der blev lodet pins på MPU6050'en. En af gruppemedlemmerne har erfaring med lodning, og lodede dem på. ESP32'eren og MPU'en blev herefter monteret på et breadboard.

![](./soldering1.png)
Lodestation med MPU6050 understøttet af en blyant.
![](./soldering2.png)
MPU6050 med pins lodet på.

Herfra forsøgte vi så hurtigt som muligt, at lave en firmware, hvor vi kunne aflæse MPU6050'erens målinger gennem I2C på ESP32-S3'eren, og sende det til host-computeren. Dette kom til at virke samme dag.

![](./images/20260324_155355.jpg)
MPU6050 måledata, sendt fra ESP32, vist på host-computer.
![](./images/20260324_155351.jpg)
Setup med breadboard og data på skærmen.

Vi kunne på nuværende tidspunkt bekræfte, at dette hardware-setup opfyldte de krav, som vi havde. Vi vurderede altså, at setup'et, som det var på breadboardet, var fyldesgørende. Dette var en af to store usikkerheder i projektet. Ved at have lagt denne usikkerhed bag os, kunne vi nu planlægge uden uvisheden af, om det overhovedet var muligt.

## Projekstyring

Vores overordnede strategi for projektstyring består af nogle hovedpunkter.
- **Punkt 1** er, at vi optimerer for forståelse og feedback. Vi prioriterer derfor, at arbejde med de svære dele af projektet, og derved bygge forståelse, en at estimere, hvor lang tid det vil tage, at lave de svære dele.
- **Punkt 2** er, at vi kun skal tage de beslutninger, som er nødvendige. Det vil eksempelvis sige, at vi ikke fastlåser os i, at skulle bruge en uge på en opgave, som med gode grunde både kunne tage 2 dage eller 2 uger. Vi lægger istedet tryk på, at være dynamiske. 
- **Punkt 3** er, at vi meget hyppigt danner overblik over processen og diskutere, hvilken retning vi bør have.

Disse punkter passer godt i vores tilfælde, dels på grund af gruppen og dels på grund af projektet. Projektet vi har valgt, har visse dele som er relativt komplicerede. Det giver en tidsmæssig usikkerhed, at inddrage systemer og teknikker, som man ikke har kompetencer i. Denne tidsmæssige usikkerhed gør det mindre brugbart at lave en udførlig plan tidligt i forløbet. Derudover er læringsprocessen sporadisk, og situationen kan derfor ændre sig uforudseeligt. Her giver det mening med dynamik i projektstyringen.

Som gruppe har vi også nogle faktorer, som øger nødvendigheden af dynamisk projektstyring. Nogle af gruppemedlemmerne inkonsistente i deres arbejdstid og -indsats. Dette kræver af projektstyringen, at det kan håndtere perioder med stor fremgang og perioder med lille fremgang. Ved hyppigt at danne overblik over projektet, er det nemmere at prioritere og estimere fra eller til i henhold til disse forhold.

## Overordnet estimering

Vi har bevidst ikke valgt, at lave en udførlig tidsplan. Istedet har vi udridset en udviklingsplan med mere fokus på tidsmæssig usikkerhed og prioritering end på tidsmæssig punktlighed.

- Lav setup med sensor og SoC-board, som opfylder vores behov. (Første uge)
- Lav fundament til 3D-spil. (Første uge)
- Lav backend-funktionalittet til kommunikation. (Første eller anden uge)
- Undersøg 3D-matematik og Kalman-filter. (Anden og tredje uge)
- Lav spil, som opfylder behov. (Anden eller tredje uge)
- Integrer system og færdiggør system (Fjerde uge)






