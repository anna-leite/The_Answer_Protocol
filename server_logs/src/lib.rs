// Server Logging: implement comprehensive logging to monitor server behavior with the
// following mandatory requirements:
// ◦ Log all client connections and disconnections with timestamps and IP addresses // .peer_addr().ip()
// ◦ Log every command received from clients with player name and parameters
// ◦ Log all server responses and error codes sent to clients
// ◦ Log world state changes (item movements, NPC interactions, combat results)
// ◦ Log quest progress and completion events
// ◦ Use structured logging format (JSON recommended) for easy parsing
// ◦ Include log levels (INFO, WARN, ERROR) for different event types
// ◦ Monitor and log potential abuse patterns (command flooding, rapid connections)
// ◦ All logs must include precise timestamps and be written to appropriate output streams
// ◦ Logging must not significantly impact server performance or responsiveness

// ==> POUR MONITOR COMMAND FLOODING:
    // creer un fonction avec un fenetre de X secodes qui enregiste les command et si la fenetre contient plus de x commands alors WARN et action pour freiner sinon saturation du server!!!! VecDeque<Instant> a voir
// ==> POUR MONITOR RAPID CONNECTION 
    // un thread par joueur ==> architecture simple.
    // creer un compteur par adresse IP pour controler le nombres de conncetions
    // on utilise la fenetre glissante des commands pour compter egalement le nombre de connexions
    // au moment de la deconnection => on nettoie proprement les ressources et on decremente.



// a voir les timestamps
    // https://rusttutorial.org/fr/date-time/
    // https://github.com/rust-lang-nursery/rust-cookbook/blob/master/src/development_tools/debugging/config_log/log-timestamp.md

// Pour des logs, la bonne pratique consiste à utiliser le format ISO 8601 en UTC (ex: 2026-11-13T11:54:00.123Z). Ce format est standard, lisible par l'humain et facile à trier ou parser par des outils externes.
// Voici les trois approches principales selon la taille de votre projet.

// 1. L'approche recommandée : les crates de logging (tracing ou env_logger)

// En production, il est préférable de ne pas générer les timestamps à la main, mais de laisser un framework de log s'en charger automatiquement avec de bonnes performances.
// • Avec tracing-subscriber (le standard moderne en Rust) :
// Ajoutez tracing et tracing-subscriber dans votre Cargo.toml.
// rust
// use tracing::{info, error};

// fn main() {
//     // Initialise le logger avec les timestamps activés par défaut (format ISO 8601 UTC)
//     tracing_subscriber::fmt::init();

//     info!("Démarrage du serveur");
//     error!(code = 500, "Échec de la connexion");
//     // Sortie : 2026-11-13T11:54:00.123456Z  INFO mon_app: Démarrage du serveur
// }
// Utilisez le code avec précaution.

// 2. L'approche légère : la crate time ou chrono (avec println!)

// Si vous écrivez vos propres logs manuellement, privilégiez la crate time (plus légère et rapide que chrono pour cet usage) ou chrono.
// • Avec chrono :
// rust
// use chrono::{Utc, SecondsFormat};

// fn log(message: &str) {
//     // SecondsFormat::Millis ajoute les millisecondes, `true` force le 'Z' pour UTC
//     let ts = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
//     println!("[{}] {}", ts, message);
// }

// fn main() {
//     log("Utilisateur connecté");
//     // Sortie : [2026-11-13T11:54:00.123Z] Utilisateur connecté
// }
// Utilisez le code avec précaution.

// 3. L'approche zéro dépendance (uniquement std)

// Si vous ne voulez aucune dépendance externe, la bibliothèque standard ne permet pas de formater une date calendaire (année/mois/jour). Vous devez vous contenter d'un timestamp numérique (en millisecondes ou nanosecondes) :
// rust
// use std::time::{SystemTime, UNIX_EPOCH};

// fn main() {
//     let ts = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_millis();

//     println!("[{}] Traitement terminé", ts);
//     // Sortie : [1794570840123] Traitement terminé
// }
// Utilisez le code avec précaution.

// Résumé des bonnes pratiques pour les logs

// • Toujours en UTC : Évite les bugs lors des changements d'heure (été/hiver) et facilite le croisement de logs entre plusieurs serveurs.
// • Précision à la milliseconde ou microseconde : Indispensable pour comprendre l'ordre exact d'événements rapides ou asynchrones.
// • Format JSON pour la production : Si vos logs sont envoyés vers Datadog, ELK ou Grafana, tracing-subscriber permet d'activer la sortie JSON en une seule ligne (.json()).