use assert_cmd::prelude::*; // Mantiene i metodi aggiuntivi come .assert()
use predicates::prelude::*; 
use std::process::Command;
// Rimuoviamo l'importazione della funzione e usiamo la macro direttamente

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    // Usiamo direttamente la macro cargo_bin! definita dal crate assert_cmd
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("grrs"));

    cmd.arg("foobar").arg("test/file/non/esistente");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Impossibile aprire"));

    Ok(())
}
