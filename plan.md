🚀 **Projekt-Blueprint: Max' KSP-Klon (Bevy 0.18 Workflow)**

Dieses Dokument dient als **Master-Leitfaden**. Es ist **linear strukturiert**, damit du von der mathematischen Basis bis hin zu den fortgeschrittenen Systemen alles in der richtigen Reihenfolge aufbaust – **ohne technische Schulden**. Alle Entscheidungen sind auf **Bevy 0.16–0.18** abgestimmt und berücksichtigen aktuelle Engine-Best-Practices.

## **🏗️ Schritt 1: Das mathematische Fundament (Woche 1\)**

Bevor du etwas siehst, muss die **Positionierung im unendlichen Raum** stimmen. Hier bekämpfen wir den *Physik-Kraken*.

### **Floating Origin Setup (big\_space)**

**Crate:** big\_space (v0.x)

**Warum:** Verhindert das Zittern der Grafik bei großen Distanzen durch Floating-Point-Ungenauigkeiten.

**Pro-Tipp:** Beginne direkt mit big\_space \+ Retained Gizmos, um stabile Orbitalmechanik *und* performante Visualisierung von Anfang an zu haben.

### **Referenzrahmen (Reference Frames) – KRITISCH**

Alle Positions- **und Geschwindigkeitswerte** sind **relativ zu einem Referenzrahmen** gespeichert. Absolute Weltkoordinaten dürfen **niemals direkt verwendet** werden.

Ein Schiff speichert Position und Velocity relativ zu seinem aktuellen Einflussbereich (SOI – *Sphere of Influence*).

enum ReferenceFrame {  
    SolarSystemBarycentric,  
    PlanetCentered(Entity), // relativ zum Planeten  
    VesselLocal(Entity),    // Docking / EVA  
}

**Wichtige Regeln:**

* Position **und** Velocity sind frame-abhängig  
* Aerodynamik nutzt Velocity relativ zur Atmosphäre  
* Docking nutzt VesselLocal Velocity  
* Beim Frame-Wechsel muss Velocity transformiert werden

v\_new \= v\_old \- v\_frame\_delta

### **Plugin-Architektur & „Zwei-Welten-Physik“**

⚠️ **KRITISCHE WARNUNG (Big Space × Avian3d):** avian3d (XPBD) ist **nicht** Floating-Origin- oder Grid-aware. Deshalb darf Physik **niemals** in globalen Koordinaten laufen.

**LÖSUNG (verpflichtend):**

* Jedes aktive Schiff besitzt eine **lokale Physik-Island**.  
* XPBD simuliert **immer relativ zum lokalen Ursprung** des aktuellen Reference Frames.  
* Beim Wechsel einer big\_space\-Grid-Cell:  
  * RigidBody wird **teleportiert ohne Impuls**  
  * Lineare & Winkelgeschwindigkeit bleiben erhalten  
  * Keine Kräfte, keine Solver-Impulse

Dadurch bleiben Orbits stabil und XPBD explodiert nicht numerisch.

**Kepler-Physik (On-Rails):**

* Planeten und passive Schiffe folgen stabilen mathematischen Bahnen f(t)  
* Interne Berechnungen immer in f64

**XPBD-Physik (Off-Rails):**

* Aktive Schiffe als RigidBody mit avian3d  
* **Physik-Islands:** XPBD wird nur im **lokalen Referenzrahmen** des Schiffs simuliert

**Hinweis:** XPBD ist nicht Floating-Origin-aware – Physik-Islands sind zwingend erforderlich.

## **🪐 Schritt 2: Die visuelle Umgebung (Woche 2\)**

Die grafische Hülle für das Sonnensystem unter Nutzung der Bevy 0.17/0.18 Rendering-Features.

### **Atmosphäre & Licht (Bevy 0.18)**

* **Raymarched Atmosphere:**  
  * AtmosphereMode::Raymarched für Weltraumansichten  
* **Atmosphere Occlusion & Lighting:**  
  * AtmosphereEnvironmentMapLight  
  * Objekte übernehmen atmosphärische Farbstiche (z.B. Sonnenuntergang)  
* **Planetenspezifische Atmosphären:**  
  * Nutzung von ScatteringMedium  
  * Unterschiedliche Streuprofile (Erde, Mars, Venus)

### **Kamera-System**

⚠️ **Rendering-Warnung (Space-Scale):** Der Standard-Z-Buffer reicht für planetare Distanzen nicht aus.

**Pflichtmaßnahmen:**

* Reverse-Z oder Logarithmic Depth Buffer  
* Kamera-Projektion auf extreme Near/Far-Ratios optimieren  
* Falls nötig: Custom Material / Shader-Pipeline

Ziel: Kein Z-Fighting zwischen Planetenoberfläche, Atmosphäre und Raumobjekten.

* Nutzung der Bevy 0.18 First-Party Camera Controller  
* Modi:  
  * **Focus Mode:** Kamera folgt dem Schiff (Orbit / Pan)  
  * **Map Mode:** Systemweite Übersicht

### **Orbit- & Debug-Visualisierung (Retained Gizmos)**

* **Retained Gizmos (Bevy 0.16+):**  
  * Statische Orbits als GizmoAsset  
  * 65–80× schneller als Immediate Gizmos  
* Darstellung von:  
  * Orbits  
  * Manöver-Nodes  
  * Velocity- & Schubvektoren

## **🏗️ Schritt 3: Modularer Fahrzeugbau (Woche 3–4)**

Bauteile, Hierarchien und persistente Konstruktionen.

### **ECS Relationships & Component Propagation**

* Parts werden über Relationships verknüpft (attached\_to)  
* Nutzung von **Component Propagation** (Bevy 0.17+):  
  * RenderLayers, Visibility, SOI-Zuordnung  
  * Automatische Weitergabe an alle Child-Entities

Kein manuelles Traversieren der Hierarchie notwendig.

### **Center of Mass (CoM)**

* Dynamische Schwerpunktberechnung  
* Masse aller Kind-Entities wird berücksichtigt  
* Kritisch für Stabilität bei leer werdenden Tanks

### **VAB – Vehicle Assembly Building**

* bevy\_mod\_picking für Drag & Drop  
* Snap-Points für valide Verbindungen

### **Asset-Workflow (glTF Extensions – Bevy 0.18)**

* Part-Daten direkt im glTF speichern:  
  * max\_thrust  
  * resource\_capacity  
  * CoM\_offset

Kein separates JSON-Mapping notwendig – Designer arbeiten direkt in Blender.

### **Persistenz**

* **Blueprints:**  
  * Nur Parts & Verbindungen  
  * Keine Physik  
  * Versionsstabil  
* **Savegames:**  
  * Orbits  
  * Ressourcenstände  
  * Aktiver Reference Frame  
  * Manöver-Nodes

Blueprints und Savegames sind **getrennte Datenformate**.

## **🚀 Schritt 4: Flugphysik, Steuerung & Aerodynamik (Woche 5–6)**

Jetzt wird es ernst: Triebwerke treffen auf Luftwiderstand.

### **Control Layer (Input ≠ Physik)**

struct ControlState {  
    throttle: f32,  
    pitch: f32,  
    yaw: f32,  
    roll: f32,  
}

* Spielerinput → ControlState  
* Autopilot / Scripts → ControlState  
* Physik liest **nur** ControlState

### **Aerodynamik-System**

* Lift & Drag pro Part  
* Velocity relativ zur Atmosphäre  
* Atmosphärenrotation berücksichtigen

F\_drag \= 0.5 \* ρ(h) \* v² \* C\_d \* A

* Lift orthogonal zu Velocity **und** Part-Orientation

### **Antrieb & XPBD**

⚠️ **Wobbly-Rocket-Falle (historisch belegt – KSP1 & KSP2):** Lange Joint-Ketten führen zu instabilen Raketen.

**Empfohlene Strategie:**

* **Standardfall:**  
  * Rakete \= *ein starrer RigidBody*  
  * Nutzung von **Compound Colliders**  
* **Flexibilität & Bruch:**  
  * Sehr steife XPBD-Joints (hohe Solver Iterations)  
  * Unsichtbare "Struts" als Zusatz-Constraints  
  * Joints primär für:  
    * Staging  
    * Zerstörung  
    * Gelenkte Module

Stabilität hat Vorrang vor physikalischer Reinheit.

* Triebwerksschub wirkt auf RigidBody  
* avian3d\-Joints für strukturelle Flexibilität & Bruch

### **Ressourcen & Observers (Bevy 0.17+)**

* ECS Observers für Events:  
  * „Tank leer“  
  * „Überhitzung“

### **Staging**

* Sequenzielles Trennen von Parents  
* Aktivierung neuer Triebwerke

## **🌌 Schritt 5: Fortgeschrittene Systeme (Woche 7+)**

## **🧠 Multiplayer & Time-Warp-Strategie (Design-Entscheidung)**

**Der kritischste Punkt für Multiplayer-Fähigkeit.**

### **Das Kernproblem**

* Spieler A: Time-Warp 1000× zum Mond  
* Spieler B: Echtzeit im VAB

Ohne klare Strategie ist Multiplayer unmöglich.

### **✅ Gewählte Lösung: Subspace / Ghost-Timeline (empfohlen)**

**Prinzip:**

* Jeder Spieler besitzt seine **eigene Zeitlinie**  
* Time-Warp erzeugt eine neue Subspace-Zeit  
* Spieler in Warp sind **nicht direkt interaktionsfähig**

**Regeln:**

* Interaktion nur, wenn:  
  * gleicher Reference Frame  
  * gleiche Zeitlinie  
* Warp-Spieler erscheinen als *Ghosts* (read-only)  
* Physik & Kollisionen nur in Echtzeit-Subspaces

### **Vorteile**

* Realistisch umsetzbar  
* Kein globaler Zeit-Zwang  
* Deterministische Orbits  
* Multiplayer bleibt optional

### **Nachteile**

* Konzeptuell anspruchsvoll  
* UI muss Zeitlinien klar visualisieren

### **Bewusst ausgeschlossen**

* ❌ Globaler Warp-Zwang  
* ❌ Voll synchronisierte Timelines  
* ❌ Physik-Rollback über Zeit

Multiplayer wird **bewusst später aktiviert**, ohne die Architektur zu brechen.

Zeitbeschleunigung, Hitze und Orbitalplanung.

### **Thermisches System**

* Temperatur **und Heat Capacity** pro Part  
* Re-Entry Heat abhängig von Geschwindigkeit & Atmosphärendichte  
* Ablation als Ressourcenverbrauch (Hitzeschilde)

### **Time-Warp & Manöver-Nodes**

* **Physikalischer Warp:**  
  * Nur bei geringer Rotation  
  * Keine Kollision  
  * Kein Aero-Einfluss  
* **On-Rails Warp:**  
  * XPBD deaktiviert  
  * Reine Kepler-Bahn  
* **Manöver-Planer:**  
  * Ghost-Orbits  
  * Fester Zeitpunkt UT  
  * Deterministisch & Multiplayer-fähig

### **Planeten-LOD (Quadtrees)**

⚠️ **Großes Risikogebiet:** Terrain-System ≈ Aufwand der gesamten Flugphysik.

**Pflichtentscheidungen:**

* Cubemap-Sphere-Projektion (keine UV-Polprobleme)  
* Dynamisches LOD per Quadtree  
* **Collider nur lokal um das aktive Schiff** generieren

Keine globalen Mesh-Collider für ganze Planeten\!

**Empfehlung:**

* Community-Ansätze evaluieren (bevy\_planet, ähnliche Crates)  
* Kein vollständiger Eigenbau, wenn vermeidbar  
* Dynamisches Terrain-LOD  
* Hohe Detailtiefe nur in Kameranähe

## **🧭 Wie du konkret anfängst (Anti-Overwhelm-Starthilfe)**

**Ziel dieses Abschnitts:** Dir helfen, vom "leeren Projekt" in einen **kontrollierten Entwicklungsfluss** zu kommen, ohne dich im Scope zu verlieren.

### **Grundregel**

❗ **Baue niemals zwei unbekannte Systeme gleichzeitig.**

Wenn etwas nicht funktioniert, musst du immer wissen, **welches System schuld ist**.

### **Phase 0: Mentale Vorbereitung (1 Abend)**

Bevor du Code schreibst:

* Akzeptiere, dass **du nicht sofort ein Spiel baust**, sondern eine **Simulation-Engine**  
* Ziel der ersten Wochen ist **Stabilität**, nicht Gameplay  
* Alles Sichtbare ist zweitrangig

Wenn du diese Phase überspringst, frustriert dich Bevy oder Physik unnötig früh.

### **Phase 1: Das „Nichts, das korrekt ist“ (Tag 1–3)**

**Ziel:** Ein Fenster mit Kamera, Zeit und Koordinaten – sonst nichts.

**Checkliste:**

* Neues Bevy-Projekt  
* big\_space integriert  
* Eine Kamera, die:  
  * korrekt gerendert wird  
  * sich bewegen lässt  
  * keine Zittereffekte zeigt

✅ Erfolgskriterium: Du kannst die Kamera Millionen Einheiten bewegen, ohne Artefakte.

**Noch NICHT:**

* Physik  
* Planeten  
* Raketen

### **Phase 2: On-Rails-Welt (Tag 4–7)**

**Ziel:** Deterministische Planetenbewegung ohne Physik.

**Vorgehen:**

* Implementiere **nur**:  
  * Zeit (UT)  
  * Kepler-Orbits (f64)  
  * Referenzrahmen (Baryzentrisch → Planet)  
* Render:  
  * einfache Kugeln  
  * Retained Gizmos für Orbits

✅ Erfolgskriterium: Ein Planet läuft stundenlang stabil auf seiner Bahn.

### **Phase 3: Erstes Physik-Island (Woche 2\)**

**Ziel:** XPBD isoliert, ohne Weltraum-Komplexität.

**Vorgehen:**

* Ein einzelnes Schiff  
* Lokaler Ursprung bei (0,0,0)  
* avian3d **ohne** big\_space-Interaktion

✅ Erfolgskriterium: Schub wirkt stabil, nichts explodiert numerisch.

### **Phase 4: Die gefährlichste Nahtstelle (Woche 2–3)**

**Ziel:** Verbindung von On-Rails ↔ Off-Rails.

* Umschalten zwischen:  
  * Kepler-Orbit  
  * XPBD-RigidBody  
* Teleport **ohne Impuls** beim Frame- / Grid-Wechsel

❗ Wenn hier etwas instabil ist: **nicht weiterbauen**.

### **Phase 5: Erst dann Gameplay**

Erst **nachdem** alles oben stabil ist:

* VAB  
* UI  
* Ressourcen  
* Aerodynamik

### **Goldene Regel für den gesamten Weg**

**Wenn etwas kaputt geht, reduziere es auf das kleinste reproduzierbare System.**

KSP-artige Spiele scheitern nicht an Ideen – sondern an fehlender Isolation.

## **🧪 Entwicklungs-Workflow & Debugging**

* **Hot Patching (Bevy 0.17+):**  
  * Live-Tuning von Physik-Parametern  
* **Retained Gizmos \+ Tracy:**  
  * Performance-Analyse von Orbit-Rendering  
* **EasyScreenshotPlugin (0.18):**  
  * Sofortiges visuelles Feedback

## **📦 Die ultimative Werkzeugkiste (Cargo.toml)**

\[dependencies\]  
bevy \= { version \= "0.18", default-features \= false, features \= \["3d", "picking", "asset\_processor"\] }

\# Physik & Raum-Skalierung  
big\_space \= "0.x"  
avian3d \= "0.x"  
bevy\_orbits \= "0.2"

\# Daten & Modding  
bevy\_save \= "0.x"  
bevy\_mod\_scripting \= { version \= "0.19", features \= \["lua"\] }  
serde \= { version \= "1.0", features \= \["derive"\] }

\[features\]  
dev \= \["bevy/hotpatching"\]

## **💡 Strategische Tipps für Max**

* **Physics Islands:** Simuliere komplexe Physik nur lokal um aktive Schiffe  
* **Retained Gizmos:** Nutze sie für alle statischen Orbits (Map View\!)  
* **Component Propagation:** Für SOI-Wechsel, Visibility & RenderLayers  
* **Reference Frame Switching:** Frame-Wechsel beim Eintritt in eine neue SOI

🔥 **Status: Produktionsreif.**

Dieses Dokument beschreibt eine **vollständige, performante und zukunftssichere KSP-ähnliche Architektur in Bevy 0.18** – bereit für echte Entwicklung.

## **📘 Fortsetzung: Referenz- & Entscheidungsanhang**

Diese Fortsetzung ist **kein linearer Bauplan**, sondern ein **Nachschlage‑ und Entscheidungsdokument**. Sie existiert bewusst getrennt vom Haupt‑Blueprint, um Überlänge zu vermeiden und trotzdem alle wichtigen Überlegungen festzuhalten.

## **🧠 Warum diese Fortsetzung existiert**

Der Haupt‑Blueprint beantwortet die Frage:

**„Was baue ich – und in welcher Reihenfolge?“**

Diese Fortsetzung beantwortet:

**„Warum ist das so entschieden – und was darf ich bewusst NICHT tun?“**

Wenn du beim Entwickeln unsicher wirst, **hier nachschlagen – nicht neu erfinden**.

## **🚧 Architektur‑No‑Gos (bitte ernst nehmen)**

### **❌ Keine globale Physik**

**Warum verboten:**

* Floating‑Point‑Drift  
* Instabile Joints  
* Nicht deterministisch

**Stattdessen:**

* Immer lokale Physik‑Islands  
* Immer relativ zum aktiven Reference Frame

### **❌ Keine absolute Welt‑Velocity**

**Warum:**

* Aero bricht  
* Docking wird unlösbar  
* SOI‑Wechsel erzeugen Energie aus dem Nichts

**Regel:**

Velocity existiert **nur relativ zu einem Frame**.

### **❌ Kein früher Multiplayer‑Code**

**Warum:**

* Verdoppelt Komplexität  
* Maskiert Physikfehler  
* Blockiert Iteration

**Erlaubt:**

* Multiplayer‑kompatible Datenmodelle  
* Ghost‑Timeline‑Design (konzeptionell)

## **🧭 Entscheidungs‑Quick‑Guide**

Wenn du festhängst, nutze diese Tabelle:

| Problem | Entscheidung |
| :---- | :---- |
| Objekt zittert | Floating Origin / Frame falsch |
| Orbit instabil | f64‑Berechnung prüfen |
| Rakete wackelt | Compound Collider statt Joints |
| Time‑Warp bricht | Zu früh Physik aktiv |
| Alles kompliziert | Zu viele Systeme gleichzeitig |

## **🧪 Empfohlene Minimal‑Tests (separat vom Spiel\!)**

Diese Tests sollten **eigene kleine Bevy‑Apps** sein.

### **Test 1: Frame‑Wechsel‑Stabilität**

* Ein Punkt  
* Zwei Frames  
* Mehrfacher Wechsel  
* Velocity bleibt konsistent

### **Test 2: Physik‑Island‑Teleport**

* XPBD‑Body  
* Teleport um große Distanz  
* Keine Kraftspikes

### **Test 3: On‑Rails ↔ Off‑Rails**

* Umschalten Orbit → Physik → Orbit  
* Energie & Bahn bleiben plausibel

## **🧠 Mentales Modell (sehr wichtig)**

Stell dir dein Spiel **nicht** als „Welt“ vor. Stell es dir vor als:

**Eine Sammlung isolierter Simulationen**,

die sich nur über klar definierte Übergänge berühren.

Wenn du dieses Modell beibehältst:

* wird Debugging möglich  
* bleibt das Projekt beherrschbar  
* scheitert es nicht wie KSP2

## **🏁 Wann du weißt, dass du richtig unterwegs bist**

Du bist auf dem richtigen Weg, wenn:

* du Systeme testest, ohne ein Spiel zu sehen  
* Bugs reproduzierbar sind  
* du Dinge löschen kannst, ohne alles zu zerstören  
* du Wochen später verstehst, **warum** etwas so gebaut ist

## **🔚 Abschluss**

Diese Fortsetzung ist **dein Sicherheitsnetz**. Wenn du müde wirst, zweifelst oder etwas „einfach mal schnell“ machen willst:

→ **hierher zurückkommen**.

Der Haupt‑Blueprint sagt dir **was** zu tun ist.

Diese Fortsetzung erinnert dich daran, **warum Disziplin hier alles ist**.

🚀 Ende der Fortsetzung
