grrs

Uno strumento a riga di comando per la ricerca di pattern testuali all'interno di file e directory, sviluppato in Rust.
Finalità del progetto

Questo tool è stato realizzato a scopo puramente didattico per approfondire la conoscenza del linguaggio Rust. Il nucleo iniziale del codice segue il tutorial ufficiale Rust CLI Book.

Rispetto al tutorial di base, sono state implementate le seguenti estensioni:

    Ricerca ricorsiva all'interno delle directory.

    Supporto per le espressioni regolari (Regex).

    Opzione per la ricerca case-insensitive.

    Output formattato con colori e numeri di riga per una migliore leggibilità.

Caratteristiche tecniche

    Efficienza: Utilizza BufReader per processare i file riga per riga, evitando di caricare l'intero contenuto in memoria RAM.

    Robustezza: Gestione degli errori tramite il crate anyhow e test di integrazione per verificare il comportamento del binario.

    Interfaccia: Gestione degli argomenti da terminale tramite clap.

Installazione

Per installare questo strumento è necessario avere il toolchain Rust e Cargo configurati sul sistema.
Bash

cargo install --git https://github.com/tuo-utente/grrs

Utilizzo

La sintassi base prevede il pattern di ricerca e il percorso (file o cartella):
Bash

grrs [OPTIONS] <PATTERN> <PATH>

Esempi

Ricerca semplice in un file:
Bash

grrs "main" src/main.rs

Ricerca ricorsiva ignorando maiuscole/minuscole:
Bash

grrs -i "errore" ./logs

Ricerca tramite espressione regolare:
Bash

grrs "^[0-9]{3}" dati.txt

Licenza

Distribuito sotto Licenza MIT. Vedere il file LICENSE per ulteriori informazioni.
