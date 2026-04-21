
# Skateboard-slope - Produktrapport

**Skateboard-slope er et singler-player-spil, hvor spilleren styrer et objekt ned af en bane og undgår forhendringer på vejen. Spillet styres af en fysisk skateboard-device.**

Løsningen består af 1) et spil implementeret som en Desktop-applikation, 2) en Skateboard-device implementeret som en embedded device men en ESP32-S3 og en MPU6050 kombineret accelerometer og gyroskop, 3) en backend-server som understøtter kommunikation over MQTT og over vores in-house TCP-protokol, og 4) en CI-opsætningen med pipelines for hvert kodeprojekt.

Koden samt yderligere materialer ligger i Github repo'et[1].

## Spillet

Spillet er implementeret som en Desktop-applikation. Applikationen er skrevet i Rust som et Cargo-projekt. Spillets brugerflade er grafisk og bruger 3D-rendering. Applikationen bruger SDL3 til 2D-rasterizering og operativsystem-IO og inhouse 3D-projektion til at rendere 3D.

Koden ligger i `game/` i repo'et.

For at køre spillet, installer Rust, SDL3 og SDL3_ttf, og kør `cargo run`.

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

#### Rendering af scene

Spillet består primært af en enkelt scene. Denne scene er bygget op med forskellige objekter. Disse objekter er defineret som structs, inkluderende `struct Skateboard`, `struct Segment`, `struct Obstacle`, `struct Ground`. Disse er alle sub-objekter på `struct Game`. I `game::Render`-metoden, renderes scenen ved at hvert objekt renderer sig selv i et `Scene`-objekt. For at renderer figure bestående af flere figure, bruges `struct ShapeGroup` struct'et. Dette er en samling a `Shape`-objekter som kan håndteres som en samlet enhed.

Når en figur renderes (normalvis i en `render` metode), instantieres et `Shape` objekt. Objektet skaleres, roteres og flyttes til den ønskede destination i scenen. Dette gøres med 3D-vektormatematik. Både `ShapeGroup` og `Scene` giver muligheder for at rotere og flytte flere objekter som en enhed.

#### Kommunikation med backenden

Kommunikation med backend'en er enkapsuleret i `struct Server`-structet. Selvom den nok skulle have heddet `Backend`, så er den ansvarlig for at opsætte og vedligeholde forbindelsen til backenden. Kommunikationen benytter vores inhouse TCP-protokel. Pt. består den af et endpoint, som opretter en stream af sensor data.

`Server`-struct'et har en metode `subscribe`, som kaldes med en callback-funktion. Denne metode registrer spillet i backend'en, sætter en datastream op, og kalder callback-funktionen for hvert sensor-measurement, der modtages fra backenden.

I spilkoden bliver disse events samlet i en event queue. Event queue'en er en FIFO-buffer implementeret med Rusts `VecDec`-kontainer. Siden event queue'en skal virke over en thread boundary, er det nødvendigt med synkronisering. Dette gøres med Rust's `Arc<Mutex<T>>` type-pattern[7].

I `Game::update`-metoden tjekkes event queue'en for, om der er blevet tilføjet nye events. Hvis der er, så tømmes queue'en, og hver værdi bruges til at styre skateboardet.

## Skateboard

Skateboard'et er implementeret med en Arduino Nano ESP32, hvori der sidder en ESP32-S3 chip. Tilkoblet ESP32'eren er en MPU6050, som er et kombineret elektronisk accelerometer og gyroskop. ESP32'eren læser værdier fra MPU6050'eren via en inhouse driver, og sender dataen til backend'en over MQTT. Skateboardet finder via WiFi til backenden via en indbygget konfiguration. Firmware'en benytter timers, preemptive multitasking og andre features fra IDF's version af FreeRTOS, til at håndtere kommunikation med MPU'en og backenden i seperate processor med tidsmæssig decoupling.

Koden ligger i `skateboard/` i repo'et.

## Backend

Backend'en er en C++-applikation som hosted via Docker Compose på en Linux-server. Backend'ens formål er at forbinde skateboardet med spillet. Dette gøres over 2 protokoller. Skateboardet kommunikerer med backenden over MQTT. Dette gøres med Mosquitto som message broker, som også kører på Linux-serveren med Docker Compose. Spillet kommunikerer med backenden over en inhouse TCP protokol.

Koden ligger i `backend/` i repo'et.

Backend'en består konceptuelt af følgende komponenter: en Linux-server, en Mosquitto-instans, en C++-applikation, herunder en TCP-server, en MQTT-client, en JSON parser og et deployment miljø.

For at deploy backend'en kør `./deploy.sh`. Eventuelt byg og upload et opdateret backend-image ved at køre `./publish.sh`.

### Linux-server

Vores backend-opsætning ligger på en server som kører Debian. Vi har lavet en opsætning med en bruger hver, dvs. en `mtk`-, `sfj`- og `tph`-bruger. Vi har sat SSH op på brugerne, så vi kan forbinde til hver vores bruger gennem SSH med public/private-nøgler. Password-authentificering er slået fra for SSH. På serveren har vi sat *sudo* op, så bruger kan køre sudo-kommandoer uden password.

Vi har installeret Docker på serveren og Docker Compose. Vores deployment fungerer ved, at filerne synkroniseres op på serveren og `sudo docker compose up -d` køres. Vi har valgt at beholde, at man skal have root access til Docker, dels fordi det ikke gør stor forskel, dels fordi så er host-miljøet agnostics for, hvilken bruger der kørte up-kommandoen, og dels fordi, der er security issues ved at give alle adgang til Docker-systemet[10].

### Mosquitto-instants

Mosquitto[8] er sat op med Docker Compose via det officielle Docker image[9]. Instansen er konfigureret med filen `deploy/mosquitto.conf`, og authorisering er konfigureret i users-filen i `deploy/mqtt_users`. For nuværende er der en enkelt bruger `test` med password'et `1234`. Mosquitto-instansen lytter på port `1883` både internt og eksternt, og så tillader den anonyme brugere. Dette betyder, at authorisering ikke er nødvendigt. I vores setup benytter vi dog stadig username/password authorisering.

Vi har valgt at bruge Mosquitto, da softwaren selv er relativ simpel. Efter at eksperimentere med RabbitMQ besluttede vi, at RabbitMQ var for advanceret til vores behov. Vi fandt ud af, at vi med meget lille energi kunne tilføje en Mosquitto-instans til vores Docker Compose-opsætning, som dækkede vores behov.

Servicen er beskrevet følgende i `docker-compose.yml`-filen:
```yaml
mosquitto:
image: docker.io/eclipse-mosquitto
ports:
  - "1883:1883"
volumes:
  - $PWD/mosquitto.conf:/mosquitto/config/mosquitto.conf
  - $PWD/mqtt_users:/mosquitto/config/mqtt_users
```

### Server-applikation

Server-applikationen er skrevet i C++, specifikt C++23, og benytter Make som buildsystem. Applikationen afhænger af libmosquitto til at forbinde til Mosquitto-instansen over MQTT. `src/`-mappen indeholder source-filerne, `tests/`-mappen indeholder unittests og `deploy/`-mappen indeholder diverse filer, som backend'en bruger til deployment-miljøet.

Makefile'en definere diverse kommandoer til at verificere, bygge og teste backenden. For at bygge applikationen til release, kør `make RELEASE=1 build/backend`. For at køre tests, kør `make test`. For at køre bygge til debugging med GDB, kør `make GDB=1 all`.

En Docker image er beskrevet i `Dockerfile`. Docker-filen beskriver 2 images, `builder` som er byggemiljøet og `runner` som er runtime-miljøet. I byggemiljøet installeret alle dependencies til at bygge og teste applikationen, eksempelvis `mosquitto-dev`, som inkluderer de headers, som C++-compileren skal bruge, for at compile kode, der bruger libmosquitto. Efter applikationen bygges, bliver testene kørt. I runtime-miljøet installeres kun de pakker, som applikationen behøver i runtime, eksempelvis `mosquitto-libs`, som kun består af libmosquitto's runtime-libraries.

Til applikationen er der defineret en test-suite af unittests. Disse tests er standalone C++-applikationer, som inkluderer alt server-applikationskoden (udover `src/main.cpp`). Testene viser success/fail via. process return codes. Test-setup'et er "limet" sammen med følgende linjer i Make-filen:
```make
test_sources = $(shell find $(test_dir) -name '*.cpp')
test_targets = $(test_sources:$(test_dir)/%.cpp=$(build_dir)/$(test_dir)/test_%)

test: $(test_targets)
	printf "%s\n" $^ | xargs -I % sh -c 'echo "- %..." && ./% && echo "- %: OK" || (echo "- %: FAILED" && exit 1)'

$(build_dir)/$(test_dir)/test_%: $(obj_dir)/tests/%.o $(objects_without_main)
	@mkdir -p $(dir $@)
	$(LD) -o $@ $(CXXFLAGS) $^ $(LDFLAGS)
```

Projektet er sat op til udvikling med Clang-værktøjerne, specific clangd-sprogserveren[11]. clangd er sat op med `compile_flags.txt`, som er en primitiv måde at fortælle clangd, hvordan den skal fortolke koden. Filen beskriver de flag, som specificeres til compiler'en, når koden kompileres (og et flag `-xc++`, som fortæller at `.h`-filer er C++ og ikke C). Derudover er der en `.clang-format`-fil, som dikterer hvordan clangd og clang-format skal formatere koden. Her har vi eksempelvis sat indent-bredde til 4 (spaces) og kolonnemaksimum til 80:
```yaml
IndentWidth: 4
ColumnLimit: 80
```

#### MQTT-klient

Server-applikationen selv indeholder 2 primære komponenter. Den første af de to er MQTT-klienten. Dette komponent er enkapsuleret i `mst::mqtt::Client`-klassen, defineret i `src/mqtt.hpp` og `src/mqtt.cpp`. Implementationen bruger libmosquitto's C-API. Vi har valgt at bruge libmosquitto til implementering af klienten, da vi ønskede et simpelt library til håndtering af det tekniske i MQTT. Vi har valgt kun at bruge MQTT, dvs. fravælge AMQP, og at bruge Mosquitto som message broker. På grund af disse 2 grunde, valgte vi at bruge libmosquitto (Mosquitto's library og C-API). MQTT understøtter arbitrær data i beskederne, men vi har valgt, at alt kommmunikation over MQTT foregår med læselig text. Komponentet udsteder et interface, så man kan publish messages og subscribe på topics:
```c++
auto client = mst::mqtt::Client(/*...*/);

client.subscribe("/my/topic", [&](std::string_view text) {
    // ...
});

client.publish("/my/topic", "message to publish");
```

#### TCP-server

Det andet komponent er en TCP-server. TCP-serveren understøtter vores inhouse protokol til at kommunikere data til spillet. TCP-serveren er enkapsuleret i `mst::server::Server`-klassen, defineret i `src/server.hpp` og `src/server.cpp`. Serveren bruger Linux's (POSIX's) indbyggede socket-API. Vi har valgt at bruge denne API, da vi har et lille behov for funktionalitet. Vi ønsker en simpel og barebones TCP-server, og derfor egner den relativt primitive socket TCP/IP-API sig godt. Derudover viste vi, at serveren kun skulle køre i et Linux-miljø. Før vi valgte socket-API'en og TCP-protokollen undersøgte vi libmicrohttp. Vi konkluderede, at en inhouse TCP protokol og socket-API'en ville være nemmest og simplest stil vores formål. Til implementeringen brugte vi *Beej's Guide to Network Programming* som reference[12].


Serveren udstiller et interface som følgende:
```c++
auto server = mst::server::Server(/*...*/);

server.notify_subscribers(/*...*/);

server.listen();
```

`Server::listen()` starter TCP-serveren, ved at lave et socket med `socket()`, binde socket'en til en port med `bind()`, sætte socket'et til at lytte til connections med `listen()`. Socket'et sættes derefter i en file descriptor-liste, og programmet sættes derefter til at vente på events i file descriptor-listen med `poll()`. Når en ny klient forbinder, vågner serveren og opretter forbindelsen med `accept()`. Med en oprettet forbindelse kan serveren og klienten sende data frem og tilbage med `recv()` og `send()`.

Pt. er der et enkelt endpoint i TCP-protokellen: `Subscribe`. Et subscribe-kald fortæller serveren, at den skal tilføje clienten til listen af klienter, der skal modtage data fra (pt. singulært) skateboardet. Klienter forventes derefter at receive data fra serveren. Med `Server::notify_subscribers()` kan backend'en sende vinkel-data til alle registrerede subscribers. Som nuværende, sker dette i MQTT subscription handleren til topic'et `/skateboard/update`. Dvs. når skateboardet publish'er data til `/skateboard/update` over MQTT, sendes det videre til alle subscribers. Serveren har funktionalitet til håndtering af afbrudte og fejlståede forbindelser.

#### JSON-parser

I backend-applikationen er der en inhouse JSON-parser. Vi valgte, at bruge vores egen JSON-parser, da vi havde brug for den ekstra performance, vi kunne få ud af en custom implementering. JSON-parseren er enkapsuleret i `mst::json::Value` og `mst::json::parse()`, og defineret i `src/json.hpp` og `src/json.cpp`. JSON-parseren er originalt et C-projekt, som vi har ported til C++23. Med en hurtig tokenizer, fleksibel parser, vel-defineret interface og simpelt query-funktionalitet, forsøger JSON-parseren at være både hurtig og nem at bruge. Vores JSON-parser er ikke 100% standards complient[13], men den opfylder vores behov. Alternativer til en inhouse implementation kunne være nlohmann/json[14] eller simdjson[15]. Et eksempel (taget fra en unittest) er følgende:
```c++
auto object = *json::parse(R"( { "rotation": -0.0123 } )");

auto query_result = *object->query(".rotation");
auto f64_value = query_result->get_f64();

ASSERT_EQ(f64_value, -0.0123);
```

## CI

Til hver af de 3 kodeprojekter er der en CI opsætning, som udfører diverse verificeringer når kode bliver *push*'et. Opsætningen er lavet med Github actions. Til hver af de 3 projekter er der sat en pipeline op, som kloner koden, bygger koden og udfører andre verificeringer. Vi benytter et custom Docker images som byggemiljøer i CI-miljøet.


[1]: https://github.com/Mercantec-GHC/h5-projekt-mst
[2]: https://wiki.libsdl.org/SDL3/FrontPage
[3]: https://docs.rs/sdl3/latest/sdl3/
[4]: https://bevy.org/
[5]: https://en.wikipedia.org/wiki/3D_projection#Mathematical_formula
[6]: https://en.wikipedia.org/wiki/Z-buffering
[7]: https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct
[8]: https://mosquitto.org/
[9]: https://hub.docker.com/_/eclipse-mosquitto/
[10]: https://docs.docker.com/engine/security/#docker-daemon-attack-surface
[11]: https://clangd.llvm.org/
[12]: https://beej.us/guide/bgnet/
[13]: https://www.json.org/json-en.html
[14]: https://github.com/nlohmann/json
[15]: https://github.com/simdjson/simdjson

