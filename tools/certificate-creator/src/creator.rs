use std;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Write};

use openssl::x509::*;
use openssl::rsa::*;
use openssl::pkey::*;
use openssl::asn1::*;
use openssl::hash::*;

/// This function will use the supplied arguments to create a public/private key pair and from
/// those create a private key file and self-signed public cert file.
pub fn run(args: super::Args) -> Result<(), ()> {
    // Public cert goes under own/
    let public_cert_path = {
        let mut path = PathBuf::from(&args.pki_path);
        path.push("own");
        ensure_dir(&path)?;
        path.push("cert.der");
        path
    };

    // Private key goes under private/
    let private_key_path = {
        let mut path = PathBuf::from(&args.pki_path);
        path.push("private");
        ensure_dir(&path)?;
        path.push("private.pem");
        path
    };

    // Create a keypair
    let pkey = {
        let rsa = Rsa::generate(args.key_size).unwrap();
        PKey::from_rsa(rsa).unwrap()
    };

    // Create an X509 cert (the public part) as a .pem
    let cert = {
        let mut builder: X509Builder = X509Builder::new().unwrap();
        let issuer_name = {
            let mut issuer_name = X509NameBuilder::new().unwrap();
            // Common name
            issuer_name.append_entry_by_text("CN", &args.common_name).unwrap();
            // Organization
            issuer_name.append_entry_by_text("O", &args.organization).unwrap();
            // Organizational Unit
            issuer_name.append_entry_by_text("OU", &args.organizational_unit).unwrap();
            // Country
            issuer_name.append_entry_by_text("C", &args.country).unwrap();
            // State
            issuer_name.append_entry_by_text("ST", &args.state).unwrap();
            issuer_name.build()
        };
        let _ = builder.set_version(2);
        let _ = builder.set_issuer_name(&issuer_name);
        builder.set_not_before(&Asn1Time::days_from_now(0).unwrap()).unwrap();
        builder.set_not_after(&Asn1Time::days_from_now(args.certificate_duration_days).unwrap()).unwrap();
        builder.set_pubkey(&pkey).unwrap();
        // Self-sign
        let _ = builder.sign(&pkey, MessageDigest::sha1());
        builder.build()
    };

    // Write the public cert
    let der = cert.to_der().unwrap();
    println!("Writing public X509 cert to {}", public_cert_path.display());
    write_to_file(&der, &public_cert_path, args.overwrite)?;

    // Write the private key
    let pem = pkey.private_key_to_pem().unwrap();
    println!("Writing private key to {}", private_key_path.display());
    write_to_file(&pem, &private_key_path, args.overwrite)?;

    Ok(())
}

/// Ensure the directory exists, creating it if necessary
fn ensure_dir(path: &Path) -> Result<(), ()> {
    if path.exists() {
        if !path.is_dir() {
            println!("{} is not a directory ", path.display());
            return Err(());
        }
    } else {
        let result = std::fs::create_dir_all(path);
        if result.is_err() {
            println!("Cannot make directories for {}", path.display());
            return Err(());
        }
    }
    Ok(())
}

/// Writes to file or prints an error for the reason why it can't.
fn write_to_file(bytes: &[u8], file_path: &Path, overwrite: bool) -> Result<(), ()> {
    if !overwrite && file_path.exists() {
        println!("File {} already exists and will not be overwritten. Use --overwrite to disable this safeguard.", file_path.display());
        return Err(())
    }

    let file = File::create(file_path);
    if file.is_err() {
        println!("Could not create file {}", file_path.display());
        return Err(());
    }
    let mut file = file.unwrap();

    let written = file.write(bytes);
    if written.is_err() {
        println!("Could not write bytes to file {}", file_path.display());
        return Err(());
    }
    Ok(())
}
