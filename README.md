# The_Answer_Protocol

==> Build a small multiplayer text adventure with a TCP server and two clients : one CLI, one GUI.
Le server communique avec les clients grace a un protocol TCP simple ou les commandes sont envoye ligne par ligne.
Developper ensuite deux clients pour utiliser ce serveur : un en ligne de commande et un avec une interface graphique 
On est libre de choisir comment implementer notre server mais on doit respecter le protocol defini par le RFC pour que notre server puisse communiquer avec des clients qu'un autre groupe a creer.

WILL DELIVER :
    -> a SERVER (implementing the complete command set from the attached protocol)
    -> a CLI client (text-based interaction)
    -> a GUI client (for a richer interface)
    -> a static WORLD DATA (for rooms, items, and NPC) YAML or JSON
    -> building tool (genre Makefile mais approprie au langage choisi, targets/commands to install dependencies, compile, run, lint, ans clean documented in README)
    -> our implementation must support every command and event defined in the protocol document!!!!!

GLOBAL RULES :
    Language ????

    networking and GUI librarie are permitted

    Encoding & Framing : TCP sockets, UTF-8, one message per line (\n terminated)

    implementation must strictly adhere to RFC specifications (ABNF syntax, error codes, and message formats)

    the server must handle disconnects gracefully and remove player state before broadcasting leave events (detecter la deconnexion, fermer/nettoyer le socket, supprimer le joueur de ses donnees, prevenir les autres clients, continuer a fonctionner normalement)

    both clients must remain responsive while receiving asynchronous events (le client ne doit pas etre bloque en attendant le server ATTENTION utliser du multithreading ou de l'I/O non bloquante ou un mecanisme asynchrone fourni par le langage)

    No persistance is required; state may reset when the server restart

MANDATORY :
    1 - implement all commands and events from the RFC

    2 - loads static world from YAML or JSON

    3 - Validates that all exits and references in the world are correct (verifier qu'un lien entre deux objet est bien valide, que l'objet existe)

    4 - Handles malformed commands with protocol-compliant error responses (gestion d'erreur des commande invalid)

    5 - Broadcasts messages without interruption if a client disconnects mid-send (gerer le cas ou le server envoie un messages grouper et que l'un des client se deconnecte entre temps = le logiciel ne doit pas planter et le messages doit bien etre envoyer a tout les client restant connectes)

    6 - DYNAMIC ITEM MANAGEMNT 
        implements a proper item system where :
            - item exists as unique instance in the world
            - taking an item removes it from the room (no duplicate)
            - dropping an item makes it available to other players
            - items can be referenced by ID or display name
            - multi-word item names are fully supported

    7 - WORLD
        provide a coherent map 
            - ROOMS : at least 8 interconnected rooms forming one or more loops, plus at least one optional branch
            - NPCs : at least 3 distinct NCP roles (dialogue, quest-giver, enemy)
            - ITEMS : at least 4 distinct items, at least 2 must be obtainable in-world
            - QUESTS : at least 2 simple quests (fetch item, defeat NPC, deliver item)
            - DESIGN CHOICE : must design quest progression mechanics, completion validation, and reward systems. Document your implementation approach in the README
            - EXPLORATION : movement must allow a full circuit (no "line_only" maps)

    8 - COMBAT SYSTEM 
        implement basic combat mechanics
            - player start with 100 health points and can be reduced throught combat
            - Enemy NPC have varying health points based on their type and difficulty
            - ATTACK commands deals damage to enemy NPC and may trigger counter-attck
            - STATUS command shows current player health and combat status
            - player with 0 HP must respawn at a safe location with reduced health
            - combat results must be logged and broadcast to relevant players
            - DESIGN CHOICE :
                must design and implement turn_based combat mechanics, including damage firmulas, initiative order, and additional combat commands (DEFEND, FLEE, ..) Document and jsutify your choices in the README

    9 - SERVER LOGGING
        - log all client connections and disconnections with timestamps and IP addresses
        - log every command received from client with player name and parameters 
        - log all server responses and error codes sent to clients
        - log world state changes (items movement, NPC interactions, combat results)
        - log quest progerss and completion events
        - Use structred logging format (JSON recommended) for easy parsing
        - include log levels (INFO, WARN, ERROR) for different event types
        - Monitor and log potential abuses patterns (command flooding, rapid connections)  
        - All log must include precise timestamp and be written to appropriate output streams
        - logging must not significantly impact server performance or responsiveness

    CLI CLIENT 
            - Connects to server and dislpays incoming messages in real time.

            - COMMAND INTERFACE CHOICE :
                    two approaches :
                    1 - sending user input commands directly to the server using RFC protocol syntax
                    2 - more user-friendly CLI interface that transates user-firendly commands into proper RFC protocol packets behind the scenes.
                    (Document choice and implementation approach in README)
            - Keeps receiving events while waiting for user input 

    GUI CLIENT 
        - graphical toolkit is allowed
        - not true GUI, juste text-based interface
        - display room details, items, NPC and exits with real-time updates
        - shows player inventory (INVENTORY command) and provides buttons for item management (TAKE, DROP)
        - handles both item ID and display names for user conviniences (ID?????)
        - updates room view automatically after TAKE/DROP operations to reflect item availability
        - separates the chat view (global, room, group) from the log view, differents chat && une fenetre pour tout les autres logs, ceux qui nen sont pas des enchanges enttre joueur.
        - provides buttons for available actions (LOOK, MOVE, TAKE, DROP, TALK, ATTACK, STATUS, QUEST, QUESTS, WHO, GROUP, QUIT)
        - show counters for players in the room and on the server.
        - enables NPC interaction through TALK command with dialogue display

Glossary :
MUDs (Multi-User Dungeons) 
CLI (interface texte, ligne de commande)
GUI (interface graphique)
TCP server (logique + gestion des donnees)
TCP protocol (moyen de communication entre les differentes parties)
line-based (les messages sont envoyes ligne par ligne)
NPC (Non-Player Character =  a character who doesn't be a player, with simple intercation : speak, fight, ...)
TCP socket (le socket permet au programme client d'envoyer et de recevoir des donnes avec le server. COTE SERVER = cree un socket qui va ecouter les connexions et si connexion client acceptee cree un socket de connection pour communiquer avec ce client & COTE CLIENT = cree un socket et l'utilise pour se connecter au server)
UTF-8 (la facon dont le texte est encode lors de sont trajet server/client)
asynchronous events (un event que le server peut envoyer a nimporte quel moment, sans que le client l'ait explicitement demande)


!!!!!!!!! Tout ce qui doit être compris par l'autre programme passe par le protocole. Tout ce qui sert à comprendre ce que fait le programme est un log.!!!!!!!!!!
???????? Include log levels (INFO, WARN, ERROR) for different event types ????????

trois type de display :
    => invite de commande et reponse server 
    => server log 
    => protocol event pour que le joueur connaisse l'etat du jeu.