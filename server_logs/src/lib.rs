// Server Logging: implement comprehensive logging to monitor server behavior with the
// following mandatory requirements:
// ◦ Log all client connections and disconnections with timestamps and IP addresses
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