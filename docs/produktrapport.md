
# Skateboard-slope - Produktrapport

**Skateboard-slope er et singler-player-spil, hvor spilleren styrer et objekt ned af en bane og undgår forhendringer på vejen. Spillet styres af en fysisk skateboard-device.**

Løsningen består af 1) et spil implementeret som en Desktop-applikation, 2) en Skateboard-device implementeret som en embedded device men en ESP32-S3 og en MPU6050 kombineret accelerometer og gyroskop, 3) en backend-server som understøtter kommunikation over MQTT og over vores in-house TCP-protokol, og 4) en CI-opsætningen med pipelines for hvert kodeprojekt.

Koden samt yderligere materialer ligger i Github repo'et[1].

## Spillet

Spillet er implementeret som en Desktop-applikation. Applikationen er skrevet i Rust som et Cargo-projekt. Spillets brugerflade er grafisk og bruger 3D-rendering. Applikationen bruger SDL3 til 2D-rasterizering og operativsystem-IO og inhouse 3D-projektion til at rendere 3D.

Koden ligger i `game/` i repo'et.

![entity relations diagram](./h5-mst-game-entity-relations.drawio.svg)

### Vindue, taster-input og 2D-rasterizering

Vi har valgt at skrive spillet i Rust. Dette har vi valgt, fordi vi alle har tidligere erfaring med at lave spil i Rust som Desktop-applikationer med SDL (SDL2). Vi har oplevet Rust som godt til Desktop-applikationer med kompleksitet og performance-krav. Diskuterede alternativer er C++ og Typescript. Siden vi ikke har lige så meget erfaring med C++ som gruppe blev dette valgt fra. Vi vurderede, at Typescript ikke passede godt til vores behov. Dele af vores applikation ligger tæt på operativsystemet i abstraktion, og vi har mindre erfaring med at udvikle med sådanne behov i Typescript end i Rust.

Spillet er en grafisk applikation, som skal kunne renderer til skærmen og reagere på input fra spilleren. Til at implementere denne funktionalitet har vi valgt at bruge library'et SDL3 (Simple Direct Medialayer)[2]. Specifikt benytter vi Rust-*crate*'en *sdl3*, som udsteder SDL3's API i et Rust-agtigt interface.[3] SDL3 er den relativt nye version af library'et, hvor SDL2 er stadig mere populært.

Vi har valgt at bruge SDL3 af flere grunde. Den første grund er, at vi har arbejdet med SDL før. Dette gjorde det nemt for os, at lave en opsætning vi kunne bruge. Vi ville derved hurtigt finde ud af, om det passede til vores behov, vi skulle skifte til noget andet. Vi konkluderede, at det passede til vores behov.

Den anden grund er, at SDL's designprincipper passer godt ind i vores problemstilling. Vi vil gerne lave en cross-platform applikation samtidig med at implementere store dele af det grafiske selv. SDL tilbyder en letvægts platformagnostisk API, som gør det nemt at lave simpel og effektiv 2D rendering og nemt at opsamle og håndtere tastatur-input. Samtidig er der lille kompleksitet bygget ind i SDL. Istedet er designprincippet at udstede primitive API'er, så det er nemt for library'ets brugere at implementere det nødvendige funktionalitet.

Alternativer til SDL3 er først og fremmest SDL2. Vi valgte, at bruge den nye version, da vi vurderede, at versionen er moden nok, og at vi gerne ville lære forskellene mellem de 2. Vi har fundet meget få forskelle. Andre alternativer kunne være *bevy*[4]. Bevy er mere *batteries included* end SDL3. Oven i mere uddybet 2D-rendering tilbyder bevy mange andre features, som vi ikke behøver. Derudover dikterer bevy arkitekturen i koden, herunder tæt kobling med bevy's ECS-system.

Vi har 2 behov, som SDL3 skal udfylde. Det første er IO-håndtering. Dvs. oprettelse af et Desktop-vindue og håndtering af applikations-events. Det andet er rendering (rasterizering) af 2D-geometri. Dvs. en måde at tegne 2D-trekanter i farver på skærmen.

Funktionalitet til 2D-renderingen er beskrevet som et Rust trait (interface) `trait Renderer` i `src/engine/game.rs`. Dette trait definerer de funktioner, vi skal bruge til at tegne geometri, hovedsageligt `fn draw_triangles`. Dette trait er implementeret for objektet (Rust struct) `struct SdlIo`, defineret i `src/game/sdl_io.rs`. Dette struct indeholder alt SDL-specifik kode. Trait'et er implementeret, så at de positioner og størrelser man passer i `Renderer`'s funktioner er normaliserede i et koordinatsystem. `SdlIo` implementation oversætter disse positioner og størrelser til reelle skærm-værdier.

![position translation](./position-translation.png)

Funktionalitet til vindue- og event-håndtering er også enkapsuleret i `struct SdlIo`. Interface'et mellem `SdlIo` og spillet er defineret i `trait engine::Game` trait'et. Dette trait definerer funktioner, som defineret af spillet og kaldet af `SdlIo`. Dette består af `fn update`, `fn render` og `fn event` funktionerne.

Vi har så vidt muligt forsøgt at enkapsulere SDL3-afhængigheden bag library-agnostiske interfaces. Dette gør at SDL3-specifik kode er begrænset til `SdlIo`. Dette gør også, at vi har et præcist interface, som beskriver vores behov. Dog introducerer det en smule kompleksitet i Rust, når man gemmer implementeringsdetaljer bag interfaces, hvis man forsøger at undgå at introducere overhead samtidig. Eksempelvis er sammenkoblingen defineret med generics og lifetimes, som kræver at man holder tungen lige i munden, når man definerer interfacet og implementerer begge sider.

### 3D-rendering

Vi vil gerne lave 3D-rendering til vores spil. Vi har valgt at implementere 3D-rendering inhouse. Grunden til dette er hovedsageligt, at vi tænkte, det ikke ville være sværre at lave selv, end at hente et library. I begge tilfælde skulle vi sætte os ind i kompleksiteterne ved 3D-rendering. 3D-renderingskoden ligger primært i `src/engine/scene.rs` og `src/engine/math.rs`.

3D-rendering, eller retter 3D-projektering er primært et matematisk problem. Vi har defineret nogle matematiske primitiver, og defineret diverse operationer, som er nødvendige for 3D-projektioner. Disse ligger i `src/engine/math.rs`. Dette inkluderer 2D-vektor `V2`, 3D-vektor `V3`, 2D- og 3D-trekanter `Triangle2` og `Triangle3` og 3x3 matrice `M3x3`. På de forskellige primitiver har vi defineret diverse matematiske operationer såsom sammenlægning og fratrækning af 3D-vektorer, gange med skalarværdi, længde af vektor, prik- og krydsprodukt, distance mellem 2 vektorer, osv. Nogle operationer er defineret som method-funktioner, eksempelvis `V3::cross`. Andre er implementeret med indbyggede Rust operatorer såsom `std::ops::Add` og `std::ops::Mul<f64>` for `V3`. Alle skalarværdier er repræsenteret med IEEE 754 double-floating point-tal, som i Rust staves `f64`, for 64-bit float.

3D-projektionen er implementeret med *Perspective Projection*[5] som funktioner på `V3` og `Triangle3` i methods ved navn `project_2d`. Følgende formel er anvendt:

![3d math](./h5-mst-game-3d-math.jpg)
![3d illustration](./h5-mst-game-3d-illustration.jpg)

Pointen med formelen er at 2D-positionerne bestemmes via forskellen mellem skærmen og punktet på z-aksen i 3D. Dvs. jo længere væk et punkt er, dvs. jo større forskellen er på z-aksen, jo tættere på midten vil punktet ligge i 2D. Dvs. objekter tæt på skærmen vises som større og objekter, der ligger længere væk, vises som minder. Det er dette, der giver effektion af 3D.

Udregninen er defineret på 3D-vektor-struct'et `V3`:

```rust
// src/engine/math.rs
pub fn project_2d(&self, camera_pos: V3) -> V2 {
    let a = *self;
    let c = camera_pos;
    let d = a - c;
    let e = V3(0.0, 0.0, 1.0);
    V2(e.2 / d.2 * d.0 + e.0, e.2 / d.2 * d.1 + e.1)
}
```

Derudover er `project_2d` også implementeret på `Triangle3`, som producerer en `Triangle2`:
```rust
// src/engine/math.rs
pub fn project_2d(&self, camera_pos: V3) -> Triangle2 {
    Triangle2(
        self.0.project_2d(camera_pos),
        self.1.project_2d(camera_pos),
        self.2.project_2d(camera_pos),
    )
}
```
Det væsentlige er her at se, at projektionen på trekantent bare er projektionen af alle punkter i trekanten.

Ovenstående giver funktionalitet for at udrenge positionerne for enkelte trekanter fra 3D til 2D. For at rendere de scener, vi skal bruge i spil, vil vi gerne kunne rendere komplicerede figure der består af flere trekanter. Vi har valgt at implementere, så vi kan rendere kasser (boxes) og plader (planes). Figurene er defineret i `src/engine/shapes.rs`. Her er de defineret en liste af *vertices* (punkter), en liste af edges (2 vertex indices hvor punkterne udgør en edge, dvs. en kant) og en liste af faces (3 vertex indices hvor punkterne udgør en face, dvs. en overflade).

Figurene skabes og håndteres med `struct Shape` struct'et. Med dette struct kan man skabe en figure ud af de prædefinerede, som så skaleres efter behov. Structet har methods, så man kan iterere over vertices, edges og faces i form af `V3`- og `Triangle3`-værdier. Formerne kan også roteres og flyttes.

For at tegne shapes, dvs. flere trekanter på en gang, har vi implementeret `struct Scene` structet. Dette er defineret i `src/engine/scene.rs`. Formålet med dette struct, er at man kan bygge en scene og så rendere den. Når scenen renderes sørger structet for at tegne alle trekanter i den rigtige rækkefølge. Trekanter udenfor skærmen og trækanter der vender væk fra skærmen renderes ikke. Den er implementeret, ved at den akkumulerer en liste af trekanter sammen med hver trekants normalvektor og farver. Når `Scene::render` kaldes, sorteres alle trekanter ift. distance fra skærmen, derefter itereres der over alle trekanter, projektioner udregnes og trekanterne tegnes med `Renderer::draw_triangle`.

#### Sortering af trekanter

Først lidt om, hvorfor trekanterne skal sorteres. Når man renderer trekanterne, tegner man som sådan alle trekanter på skærmen. Dvs. hvis 2 eller flere trekanter overlapper hinanden i 2D-projektionen, så er vi nødt til at vide, hvilken trekant vi skal tegne først. Vi vil gerne gøre, så at den tættest på liggende trekant tegnes sidst, så den er forest på det renderede billede.

For at sortere trekanterne, udregnes en score for hver trekant. En trekants score er regnet, ved at regne hvert punkts afstand til kamera'et og så ganges distances for de to tætteste punkter. Dette er koden, som foretager denne beregning og sortering:

```rust
// src/engine/scene.rs
let mut indices_with_scores = self
    .tris
    .iter()
    .enumerate()
    .map(|(i, (tri, ..))| {
        let mut p_scores = [tri.0, tri.1, tri.2]
            .map(|p| (camera_pos - p).len());

        p_scores.sort_by(|a, b| a.total_cmp(b));

        let score = p_scores[0] * p_scores[1];
        (i, score)
    })
    .rev()
    .collect::<Vec<_>>();

indices_with_scores.sort_by(|a, b| b.1.total_cmp(&a.1));
```

Der der er værd at se, er at `p_scores`, som er hvert punkts score, udregnes ved at regne hvert punkts afstand til kameraet. Dernæst sorteret `p_scores`, så de tætteste punkter ligger i `[0]` og `[1]`, som derefter bruges til at regne den totale score. Til sidst sorteres `indices_with_scores`, så den længst væk liggende trekant ligger først.

Denne algoritme til at beregne score er vi kommet frem til gennem eksperimentering. Om dette er den optimale algoritme, ved vi ikke. Et populært alternative til at benytte en algoritme på denne måde er Z-buffering[6]. Her beregnes afstanden for hvert enkelt pixel, og man opnår derved perfekt rendering af overlappende trekanter. Ulempen ved Z-buffering er, at det er beregningstungt. I realtidsapplikationer (såsom et spil) kan det derfor ikke svare sig, hvis man laver 3D-udregninerne på CPU'en. Det er ofte, at man foretager 3D-beregninerne på GPU'en istedet. Det gør vi ikke af flere årsager, så det behøver vi ikke at bekymre os om. Istedet er vores metode, at udregne en aggrigat-værdi for hver trekant. Dvs. istedet for en afstandsberegning for hvert pixel, laver vi 3 afstandsberegninger for hver trekant og en sortering af de 3 værdier.

#### Filtering af trekanter

Ikke alle trekanter skal tegnes på skærmen. Trekanter som vender væk fra kameraet, eksempelvis bagsiden af en kasse, bør ikke tegnes. Trekanter, hvis punkter ligger bag kameraet bør heller ikke tegnes. Dels vil de ikke være på skærmen, men matematikken bliver også utilregnelig når punkterne ligger meget tæt på eller bag kameraret. Derudover sparer vi også noget CPU-tid, ved at vælge trekanter fra generelt.

For at filtrere trekanter fra, som ligger bag kameraet, benytter vi, at scenens kameraretning er fastlåst i retning af z-aksen. Dette gør, at vi bare kan tjekke hvis et af punkter ligger bag kameraet på z-aksen. Følgende kode udfører denne beregning, og hopper fremad i loop'et, hvis det er relevant:

```rust
// src/engine/scene.rs
for v in [tri3.0, tri3.1, tri3.2] {
    if v.2 < camera_pos.2 {
        continue 'render_tri_loop;
    }
}
```

For at filtrere trekanter fra, som vænder væk fra kameraet, benytter vi trekanternes normalvektor. Hvis prikproduktet af normalvektoren og afstanding mellem et af punkterne og kameraret er negativt, så ved vi, at trekanten vender i samme retning som kameraet. Vi kan derfor springe sådanne trekanter over. Dette er koden, som laver denne håndtering:

```rust
// src/engine/scene.rs
if normal.dot(camera_pos - tri3.0) < 0.0 {
    continue;
}
```

## Skateboard

Skateboard'et er implementeret med en Arduino Nano ESP32, hvori der sidder en ESP32-S3 chip. Tilkoblet ESP32'eren er en MPU6050, som er et kombineret elektronisk accelerometer og gyroskop. ESP32'eren læser værdier fra MPU6050'eren via en inhouse driver, og sender dataen til backend'en over MQTT. Skateboardet finder via WiFi til backenden via en indbygget konfiguration. Firmware'en benytter timers, preemptive multitasking og andre features fra IDF's version af FreeRTOS, til at håndtere kommunikation med MPU'en og backenden i seperate processor med tidsmæssig decoupling.

Koden ligger i `skateboard/` i repo'et.

## Backend

Backenden er en C++-applikation som hosted via Docker Compose på en Linux-server. Backend'ens formål er at forbinde skateboardet med spillet. Dette gøres over 2 protokoller. Skateboardet kommunikerer med backenden over MQTT. Dette gøres med Mosquitto som message broker, som også kører på Linux-serveren med Docker Compose. Spillet kommunikerer med backenden over en inhouse TCP protokol.

Koden ligger i `backend/` i repo'et.

## CI

Til hver af de 3 kodeprojekter er der en CI opsætning, som udfører diverse verificeringer når kode bliver *push*'et. Opsætningen er lavet med Github actions. Til hver af de 3 projekter er der sat en pipeline op, som kloner koden, bygger koden og udfører andre verificeringer. Vi benytter et custom Docker images som byggemiljøer i CI-miljøet.


[1]: https://github.com/Mercantec-GHC/h5-projekt-mst
[2]: https://wiki.libsdl.org/SDL3/FrontPage
[3]: https://docs.rs/sdl3/latest/sdl3/
[4]: https://bevy.org/
[5]: https://en.wikipedia.org/wiki/3D_projection#Mathematical_formula
[6]: https://en.wikipedia.org/wiki/Z-buffering

