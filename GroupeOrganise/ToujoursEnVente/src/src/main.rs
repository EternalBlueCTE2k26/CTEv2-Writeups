//! # Consignes
//!
//! reverse Facile voir Moyen,
//! Le challenge tourne autour de l’entité FakeBrokeur de fantasmas de redes
//! Il s’agit d’une entité en charge de faire l’aquisition de grandes quantité de données personnels à partir de :
//! achat de listing auprès d’entreprise qui revend des donnés,
//! d’entreprise qui font faillite et revendre leur listing client,
//! achat de donnés de leak sur le darkweb suite à un refus de paiement de rançon,
//! L’idéal dans ce challenge est de découvrir à l’issu du reverse un texte caché ou coder qui recompte un peut leur activités illicite et un flag cacher dans le texte
//! Par exemple : le compte Telegram sur lequel les revendeurs doivent se rendre pour échanger avec eux au sujet d’une Neto de vente de données volées
//! t.me/FakeBrokeurs
//! donc l'objectif de ton challenge c'est de découvrir après l'opération du reverse  l'url telegram qui mène au group public FakeBrokeurs
//! Nom du challenge : Toujours en vente
//! Nb points  : 150
//!
//!
//! # Idées
//!
//! texte à afficher dans un menu
//! d'abord en access denied
//! ce texte n'est pas directement dans le binaire, pour éviter qu'on le trouve tout de suite
//! on le XOR avec un OTP, pour pouvoir dire que c'est secure
//! maintenant faut aider le jouer à construire l'OTP (One-Time Pad, chiffrement "parfait")
//! il faut ajouter l'histoire
//! et du bruit
//!
//! # TODO
//!
//! Séparer en fichiers pour éviter d'avoir tout dans ce main.rs, ce qui n'est pas truc idiomatique...

use std::env;
use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        cls();
        match menu() {
            Some(Menu::Item(opt)) => match opt {
                1 => submit_mails(),
                2 => submit_leaked_photo(),
                3 => submit_leaked_doc(),
                4 => bug_report(),
                _ => println!("unknown menu"),
            },
            Some(Menu::Quit) => break,
            None => println!("invalid input"),
        }
        thread::sleep(Duration::from_millis(2000));
        read_line("\nPRESS ENTER TO MENU");
    }
}

fn cls() {
    print!("\x1Bc"); // clear screen, probably only on linux terms ... and we compile to linux only
}

/// Read a line from stdin without the trailing \n
fn read_line(prompt: &str) -> Option<String> {
    print!("{prompt}");
    let _n = io::stdout().flush().ok()?; // Rust is line-buffered by default

    let mut s = String::new();
    let _n = io::stdin().read_line(&mut s).ok()?;

    s.truncate(s.trim_end().len());
    Some(s)
}

// FakeBrokeurs portal - a dark marketplace for data
const MENU: &str = "\
===================================
   B U S I N E S S   P O R T A L   
===================================

Welcome, valued partner.
Remember: data is our business. And business is GOOD.

1. Submit Email Lists
2. Submit Leaked Photos
3. Submit Leaked Documents
4. Bug Report

q. Quit";

fn menu() -> Option<Menu> {
    // Do it in a single call to simplify the debugging process
    println!("{MENU}");

    let s = read_line("Choose: ")?;
    println!();

    if s == "q" || s == "Q" {
        Some(Menu::Quit)
    } else {
        Some(Menu::Item(s.parse::<usize>().ok()?))
    }
}

enum Menu {
    Item(usize),
    Quit,
}

/// Distraction menu
#[inline(never)]
fn submit_mails() {
    cls();
    println!("=== SUBMIT EMAIL LISTS ===");
    println!();
    println!("So you have email lists to sell?");
    println!("We LOVE fresh data. The staler, the better.");
    println!("Companies that went bankrupt? Their loss is OUR gain.");
    println!("Just remember: we verify EVERYTHING. Don't try to scam us.");
    println!();
    println!("Categories we accept:");
    println!("- corporate mailing lists (min 10k emails)");
    println!("- e-commerce customer databases");
    println!("- newsletter subscriptions");
    println!();

    // Ask for the emails
    loop {
        let input = read_line("Enter the email (leave empty to finish): ");
        if input.map_or(true, |s| s.trim().is_empty()) {
            break;
        }
    }
    println!();

    validation();
}

/// Fake validation animation using \r and ...
fn validation() {
    for _ in 0..6 {
        print!("\rValidating   \r");
        print!("Validating");
        io::stdout().flush().ok();
        for _ in 0..3 {
            print!(".");
            io::stdout().flush().ok();
            thread::sleep(Duration::from_millis(100));
        }
    }
    println!();
    println!();
    println!("ERROR, invalid format. Stop being annoying.");
    println!("Do not try to reach us. We know where to find you.");
}

/// Distraction menu
#[inline(never)]
fn submit_leaked_photo() {
    cls();
    println!("=== SUBMIT LEAKED PHOTOS ===");
    println!();
    println!("Ah, visual evidence. The currency of the modern age.");
    println!("Whether from ransomware victims who couldn't pay,");
    println!("or disgruntled employees with access to sensitive systems...");
    println!("We value EVERY contribution to our collection.");
    println!();
    println!("Requirements:");
    println!("- EXIF data preferred (we extract metadata)");
    println!("- documents, IDs, credentials in photo form");
    println!("- screenshots of internal systems");
    println!();
    read_line("Enter origin and identified people: ");
    read_line("File path to image: ");
    println!();

    validation();
}

/// Distraction menu
#[inline(never)]
fn submit_leaked_doc() {
    cls();
    println!("=== SUBMIT LEAKED DOCUMENTS ===");
    println!();
    println!("Documents. The crown jewels of data brokerage.");
    println!("From companies that thought their backups were safe,");
    println!("to executives who used 'password123' on their accounts.");
    println!("We extract value where others only see trash.");
    println!();
    println!("High-value targets:");
    println!("- financial records and spreadsheets");
    println!("- HR databases with employee info");
    println!("- contract and legal documents");
    println!("- source code and intellectual property");
    println!();
    read_line("Enter document type and origin: ");
    read_line("File path to image: ");
    println!();

    validation();
}

/// The password-protected menu item
#[inline(never)]
fn bug_report() {
    // Get the password from somewhere (env var?)
    let maybe_var = env::var("ADMIN_PWD");
    if maybe_var.is_err() {
        // Hint the user to search how to give a password
        //  (we could fail silently by providing a default password)
        println!("you need the password to unlock this menu");
        return;
    }

    // Work on the bytes to simplify reverse eng
    let var = maybe_var.unwrap();
    let pwd = var.as_bytes();

    // Pretend that we don't want to bother decrypting if the password is wrong
    //  (also avoid printing unknown things on the console)
    // But in fact we need to extract the decryption key from it
    let maybe_key = validate_password(pwd);
    if let Err(msg) = maybe_key {
        if msg.len() > 0 {
            println!("{msg}");
        }
        println!("invalid password: cannot decrypt message");
        return;
    }

    // Decrypt & print
    let key = maybe_key.unwrap();
    if let Some(msg) = decrypt(key.as_slice()) {
        println!("{}", msg);
    } else {
        println!("invalid password: cannot decrypt message");
    }
}

/// The function that needs to be bypassed,
///  the rest is only decorative.
/// The function returns the real password used for decryption
///  which, of course, has more constraints because it must be unique, hence correctly guessed.
///
/// The final password should be of sufficient length to dissuade from bruteforce,
///  e.g. 16 chars with all symbols.
/// Operations, not necessarily in this order:
/// - check min length,
/// - check max length,
/// - check length mod 3 then check length mod 4,
/// - some of the tests should be in an instance to add a level of indirection
/// - sha256(password) with some bits 1 or 0 (proof of work)
///   -> a zone should be free of constraint to put a nonce,
///   -> MAY BE TOO MUCH, make it a low constraint (high probability),
/// - x times there should be a letter like the penultimate one A-ABAB--
/// - x times there should be a digit followed by its +1 (01, 45, ...)
/// - ! should be in the middle
/// - { and } should be there once and { before }
/// - there must not be more than x consecutive small case letters
/// - there must be F4n745masD3Red3s inside the password (of course, we have to hide it),
///   not necessarily in one chunk,
/// - abre-te, Sésamo,
/// - pasteis de nata e bacalhau,
/// - sum of values should be 0 % 5,
/// - 4 lower 4 upper 4 digits 4 special
#[inline(never)]
fn validate_password(pwd: &[u8]) -> Result<Vec<u8>, String> {
    if !pwd.is_ascii() {
        // This one is quiet
        return Err(String::new());
    }

    if pwd.len() < 16 {
        return Err(String::from("the given password is too short"));
    }

    validate_len_mod(pwd, 3)?;
    validate_len_mod(pwd, 4)?;
    validate_sum_mod(pwd, 5)?;

    let mut validator = PwdValidation::new();
    validator.process_pwd(pwd);
    validator.validate()?;

    Ok(validator.result) // Move result out of the struct
}

fn validate_len_mod(pwd: &[u8], modulus: usize) -> Result<(), String> {
    if (pwd.len() % modulus) != 0 {
        Err(format!("password length must be a multiple of {modulus}"))
    } else {
        Ok(())
    }
}

fn validate_sum_mod(pwd: &[u8], modulus: u64) -> Result<(), String> {
    let mut sum = 0_u64;
    for c in pwd.iter() {
        sum = sum
            .checked_add(*c as u64)
            .ok_or(String::from("overflow when summing password"))?;
    }

    if (sum % modulus) != 0 {
        Err(format!("{ERR_SUM_MOD}{modulus}"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests_validations {
    use super::*;

    #[test]
    fn invalid_main() {
        assert_eq!(validate_password(b"\xE9"), Err(String::new()));
        assert_eq!(
            validate_password(b"AB"),
            Err(String::from("the given password is too short"))
        );
    }

    #[test]
    fn len_mod() {
        // Successes
        validate_len_mod(b"123", 3).unwrap();
        validate_len_mod(b"123456", 3).unwrap();
        validate_len_mod(b"123456789abc", 3).unwrap();
        validate_len_mod(b"1234", 4).unwrap();
        validate_len_mod(b"12345678", 4).unwrap();
        validate_len_mod(b"123456789abc", 4).unwrap();
        // Failures
        validate_len_mod(b"12", 3).unwrap_err();
        validate_len_mod(b"1234", 3).unwrap_err();
        validate_len_mod(b"12", 4).unwrap_err();
        validate_len_mod(b"123", 4).unwrap_err();
        validate_len_mod(b"12345", 4).unwrap_err();
    }

    #[test]
    fn sum_mod() {
        // Successes
        validate_sum_mod(b"\x00\x00\x00", 5).unwrap();
        validate_sum_mod(b"\x00\x04\x01", 5).unwrap();
        validate_sum_mod(b"AAA", 5).unwrap();
        validate_sum_mod(b"BBBBB", 5).unwrap(); // 5 times the same chars should always validate

        // Failures
        validate_sum_mod(b"\x00\x04\x01", 6).unwrap_err();
        validate_sum_mod(b"\x00\x04\x02", 5).unwrap_err();
        validate_sum_mod(b"AAA", 6).unwrap_err(); // A=0x41=65
    }

    #[test]
    fn a_good_password() {
        // Passes all steps
        // Length 48 = 4*3*4
        // Sum(ABCDEFGHIJKL!p!a!s!t!e!!s !d!3! na74 3 b4c4lh4u) is 3 and \ is 0x5c == 92
        validate_password(b"ABCDEFGHIJKL!p!a!s!t!e!s! !d!3! na74 3 b4c4lh4u\\").unwrap();
    }
}

/// Group some validations inside a structure that will just have some flags raised
///  so that we can stop re-iterating on the bytes and make for little more challenging validations
#[derive(PartialEq, Debug)]
struct PwdValidation {
    flag_1lower: bool,
    flag_1upper: bool,
    flag_1digit: bool,
    flag_1special: bool,
    flag_bang: bool,
    remaining_lower: u8,
    remaining_upper: u8,
    remaining_same_penults: u8,
    remaining_increments: u8,
    secret_index: usize,
    secret_passed: bool,
    result: Vec<u8>,
}

impl PwdValidation {
    fn new() -> PwdValidation {
        PwdValidation {
            flag_1lower: false,
            flag_1upper: false,
            flag_1digit: false,
            flag_1special: false,
            flag_bang: false,
            remaining_lower: 9,
            remaining_upper: 5,
            remaining_same_penults: 7,
            remaining_increments: 6,
            secret_index: 0,
            secret_passed: false,
            result: Vec::new(),
        }
    }

    fn process_pwd(&mut self, pwd: &[u8]) {
        let len = pwd.len();
        for (i, c) in pwd.iter().enumerate() {
            match c {
                0x21 => {
                    self.flag_1special = true;
                    if i == len / 2 {
                        self.flag_bang = true;
                    }
                }
                0x30..=0x39 => self.flag_1digit = true,
                0x41..=0x5a => {
                    self.flag_1upper = true;
                    if self.remaining_upper > 0 {
                        self.remaining_upper -= 1;
                    }
                }
                0x61..=0x7a => {
                    self.flag_1lower = true;
                    if self.remaining_lower > 0 {
                        self.remaining_lower -= 1;
                    }
                }
                // Almost all the rest of the ASCII table is ok for symbols
                0x22..=0x7e => self.flag_1special = true,
                _ => {}
            }
            // check xyzAuA
            if self.remaining_same_penults > 0 && i >= 2 && pwd[i - 2] == *c {
                self.remaining_same_penults -= 1;
            }
            // check XYZ or 0123 or dcABzy
            if self.remaining_increments > 0 && i >= 1 && pwd[i - 1] == *c - 1 {
                self.remaining_increments -= 1;
            }
            if !self.secret_passed
                && SECRET.xored_msg[self.secret_index] ^ (SECRET.key + (self.secret_index as u8))
                    == *c
            {
                self.result.push(*c);
                self.secret_index += 1;
                self.secret_passed = self.secret_index == SECRET.xored_msg.len();
            }
        }
    }

    fn validate(&self) -> Result<(), String> {
        if !self.flag_1lower {
            return Err(String::from("there should be at least 1 lower case letter"));
        }
        if !self.flag_1special {
            return Err(ERR_1SPECIAL.to_string());
        }
        if !self.flag_bang {
            return Err(ERR_BANG.to_string());
        }
        if self.remaining_lower > 0 {
            return Err(ERR_REM_LOWER.to_string());
        }
        if !self.flag_1upper {
            return Err(String::from("there should be at least 1 upper case letter"));
        }
        if self.remaining_upper > 0 {
            return Err(ERR_REM_UPPER.to_string());
        }
        if self.remaining_same_penults > 0 {
            return Err(ERR_REM_SAME.to_string());
        }
        if !self.flag_1digit {
            return Err(ERR_1DIGIT.to_string());
        }
        if self.remaining_increments > 0 {
            return Err(ERR_REM_INCS.to_string());
        }
        // This one is quiet
        if !self.secret_passed {
            Err(String::new())
        } else {
            //Ok(self.result) // This should move my result
            // Of course, we can't move the result,
            //  because it kinds of consumes the struct
            // So we could have an into_key(self) -> Result<Vec<u8>, String>
            //  but then it makes tests harder and this is just *one* challenge
            Ok(()) // We will move it from the outside, which has the object instead of a ref
        }
    }
}

#[cfg(test)]
mod tests_pwd_validator {
    use super::*;

    #[test]
    fn validation() {
        let validator = PwdValidation::new();
        validator.validate().unwrap_err();
        let validator = PwdValidation {
            flag_1lower: true,
            flag_1upper: true,
            flag_1digit: true,
            flag_1special: true,
            flag_bang: true,
            remaining_lower: 0,
            remaining_upper: 0,
            remaining_same_penults: 0,
            remaining_increments: 0,
            secret_index: 0,
            secret_passed: true,
            result: vec![1, 2, 3],
        };
        validator.validate().unwrap();
        assert_eq!(validator.result, vec![1, 2, 3]); // ...
    }

    #[test]
    fn examples() {
        let mut validator = PwdValidation::new();
        validator.process_pwd(b"a");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true, //
                flag_1upper: false,
                flag_1digit: false,
                flag_1special: false,
                flag_bang: false,
                remaining_lower: 8, //
                remaining_upper: 5,
                remaining_same_penults: 7,
                remaining_increments: 6,
                secret_index: 0,
                secret_passed: false,
                result: vec![],
            }
        );
        validator.process_pwd(b"A");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true,
                flag_1upper: true, //
                flag_1digit: false,
                flag_1special: false,
                flag_bang: false,
                remaining_lower: 8,
                remaining_upper: 4, //
                remaining_same_penults: 7,
                remaining_increments: 6,
                secret_index: 0,
                secret_passed: false,
                result: vec![],
            }
        );
        validator.process_pwd(b"0");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true,
                flag_1upper: true,
                flag_1digit: true, //
                flag_1special: false,
                flag_bang: false,
                remaining_lower: 8,
                remaining_upper: 4,
                remaining_same_penults: 7,
                remaining_increments: 6,
                secret_index: 0,
                secret_passed: false,
                result: vec![],
            }
        );
        validator.process_pwd(b"*");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true,
                flag_1upper: true,
                flag_1digit: true,
                flag_1special: true, //
                flag_bang: false,
                remaining_lower: 8,
                remaining_upper: 4,
                remaining_same_penults: 7,
                remaining_increments: 6,
                secret_index: 0,
                secret_passed: false,
                result: vec![],
            }
        );
        // No change as ! is not in the middle
        validator.process_pwd(b"-*!");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true,
                flag_1upper: true,
                flag_1digit: true,
                flag_1special: true,
                flag_bang: false,
                remaining_lower: 8,
                remaining_upper: 4,
                remaining_same_penults: 7,
                remaining_increments: 6,
                secret_index: 0,
                secret_passed: false,
                result: vec![],
            }
        );
        validator.process_pwd(b"-+!7");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true,
                flag_1upper: true,
                flag_1digit: true,
                flag_1special: true,
                flag_bang: true, //
                remaining_lower: 8,
                remaining_upper: 4,
                remaining_same_penults: 7,
                remaining_increments: 6,
                secret_index: 0,
                secret_passed: false,
                result: vec![],
            }
        );
        // Should not trigger same_penults not increments, nor secret
        validator.process_pwd(b"zyxwvutsrqaffaZYXWVUTSRQPONMAFFA");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true,
                flag_1upper: true,
                flag_1digit: true,
                flag_1special: true,
                flag_bang: true,
                remaining_lower: 0, //
                remaining_upper: 0, //
                remaining_same_penults: 7,
                remaining_increments: 6,
                secret_index: 0,
                secret_passed: false,
                result: vec![],
            }
        );
        validator.process_pwd(b"affaAaf!f!A");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true,
                flag_1upper: true,
                flag_1digit: true,
                flag_1special: true,
                flag_bang: true,
                remaining_lower: 0,
                remaining_upper: 0,
                remaining_same_penults: 4, //
                remaining_increments: 6,
                secret_index: 0,
                secret_passed: false,
                result: vec![],
            }
        );
        validator.process_pwd(b"{||}afbcejklo");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true,
                flag_1upper: true,
                flag_1digit: true,
                flag_1special: true,
                flag_bang: true,
                remaining_lower: 0,
                remaining_upper: 0,
                remaining_same_penults: 4,
                remaining_increments: 1, //
                secret_index: 0,
                secret_passed: false,
                result: vec![],
            }
        );
        validator.process_pwd(b"[p]-as$t_!%el!");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true,
                flag_1upper: true,
                flag_1digit: true,
                flag_1special: true,
                flag_bang: true,
                remaining_lower: 0,
                remaining_upper: 0,
                remaining_same_penults: 4,
                remaining_increments: 1,
                secret_index: 6, //
                secret_passed: false,
                result: b"paste!".to_vec(), //
            }
        );
        validator.process_pwd(b"paste!s d3 na74 3 b4c4lh4u");
        assert_eq!(
            validator,
            PwdValidation {
                flag_1lower: true,
                flag_1upper: true,
                flag_1digit: true,
                flag_1special: true,
                flag_bang: true,
                remaining_lower: 0,
                remaining_upper: 0,
                remaining_same_penults: 2, // side effect of _3_ and b4c4
                remaining_increments: 0,   // side effect of st in paste!s
                secret_index: 26,          //
                secret_passed: true,       //
                result: b"paste!s d3 na74 3 b4c4lh4u".to_vec(), //
            }
        );
    }

    #[test]
    // The correct solution decryption key should not be validated
    fn direct_secret() {
        let mut validator = PwdValidation::new();
        validator.process_pwd(b"paste!s d3 na74 3 b4c4lh4u");
        assert!(validator.secret_passed);
        validator.validate().unwrap_err();
    }

    #[test]
    fn a_good_password() {
        let mut validator = PwdValidation::new();
        validator.process_pwd(b"ABCDEFGHIJKL!p!a!s!t!e!!s !d!3! na74 3 b4c4lh4u");
        validator.validate().unwrap();
    }
}

/// The concept of hiding error means:
/// 1. XOR with a key at compilation
/// 2. XOR with the key at runtime to show the message
/// As I'm not skilled enough to write a macro that would do that at compile time,
///  I'm using a small python program to preprocess the string literals
struct XoredLiteral<'a> {
    key: u8,
    xored_msg: &'a [u8],
}

impl XoredLiteral<'_> {
    const fn from_xored(key: u8, xored_msg: &[u8]) -> XoredLiteral<'_> {
        XoredLiteral {
            key: key,
            xored_msg: xored_msg,
        }
    }
}

use std::fmt;

impl fmt::Display for XoredLiteral<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            str::from_utf8(
                &self
                    .xored_msg
                    .iter()
                    .map(|c| c ^ self.key)
                    .collect::<Vec<u8>>()
            )
            // Only fails on partial UTF-8 chars
            //  (other weird chars are displayed as \u{...})
            .unwrap_or("[error displaying error]")
        )
    }
}

#[cfg(test)]
mod tests_xored {
    use super::*;

    #[test]
    fn unxor() {
        assert_eq!(
            "test error message",
            XoredLiteral::from_xored(0x40, b"4%34`%22/2`-%33!'%").to_string()
        );
        assert_eq!(
            "test error message",
            XoredLiteral::from_xored(
                0x72,
                b"\x06\x17\x01\x06R\x17\x00\x00\x1d\x00R\x1f\x17\x01\x01\x13\x15\x17"
            )
            .to_string()
        );
        assert_eq!(
            "test error message",
            XoredLiteral::from_xored(
                0xAA,
                b"\xde\xcf\xd9\xde\x8a\xcf\xd8\xd8\xc5\xd8\x8a\xc7\xcf\xd9\xd9\xcb\xcd\xcf"
            )
            .to_string()
        );
    }
}

// "sum of bytes in password must be a multiple of "
const ERR_SUM_MOD: XoredLiteral = XoredLiteral::from_xored(0x67, b"\x14\x12\nG\x08\x01G\x05\x1e\x13\x02\x14G\x0e\tG\x17\x06\x14\x14\x10\x08\x15\x03G\n\x12\x14\x13G\x05\x02G\x06G\n\x12\x0b\x13\x0e\x17\x0b\x02G\x08\x01G");
// "there should be at least 1 digit"
const ERR_1DIGIT: XoredLiteral = XoredLiteral::from_xored(
    0x37,
    b"C_RER\x17D_XB[S\x17UR\x17VC\x17[RVDC\x17\x06\x17S^P^C",
);
// "there should be at least 1 symbol"
const ERR_1SPECIAL: XoredLiteral = XoredLiteral::from_xored(
    0x27,
    b"SOBUB\x07TOHRKC\x07EB\x07FS\x07KBFTS\x07\x16\x07T^JEHK",
);
// "a ! should be placed in the middle"
const ERR_BANG: XoredLiteral = XoredLiteral::from_xored(0xa0, b"\xc1\x80\x81\x80\xd3\xc8\xcf\xd5\xcc\xc4\x80\xc2\xc5\x80\xd0\xcc\xc1\xc3\xc5\xc4\x80\xc9\xce\x80\xd4\xc8\xc5\x80\xcd\xc9\xc4\xc4\xcc\xc5");
// "there are not enough lower case letters"
const ERR_REM_LOWER: XoredLiteral = XoredLiteral::from_xored(0xf4, b"\x80\x9c\x91\x86\x91\xd4\x95\x86\x91\xd4\x9a\x9b\x80\xd4\x91\x9a\x9b\x81\x93\x9c\xd4\x98\x9b\x83\x91\x86\xd4\x97\x95\x87\x91\xd4\x98\x91\x80\x80\x91\x86\x87");
// "there are not enough upper case letters"
const ERR_REM_UPPER: XoredLiteral = XoredLiteral::from_xored(0xea, b"\x9e\x82\x8f\x98\x8f\xca\x8b\x98\x8f\xca\x84\x85\x9e\xca\x8f\x84\x85\x9f\x8d\x82\xca\x9f\x9a\x9a\x8f\x98\xca\x89\x8b\x99\x8f\xca\x86\x8f\x9e\x9e\x8f\x98\x99");
// "more chars must be the same as the penultimate"
const ERR_REM_SAME: XoredLiteral = XoredLiteral::from_xored(0xb7, b"\xda\xd8\xc5\xd2\x97\xd4\xdf\xd6\xc5\xc4\x97\xda\xc2\xc4\xc3\x97\xd5\xd2\x97\xc3\xdf\xd2\x97\xc4\xd6\xda\xd2\x97\xd6\xc4\x97\xc3\xdf\xd2\x97\xc7\xd2\xd9\xc2\xdb\xc3\xde\xda\xd6\xc3\xd2");
// "more chars must be the increment of the previous char (ABC...)"
const ERR_REM_INCS: XoredLiteral = XoredLiteral::from_xored(
    0x14,
    b"y{fq4w|ufg4yag`4vq4`|q4}zwfqyqz`4{r4`|q4dfqb}{ag4w|uf4<UVW:::=",
);
// "paste!s d3 na74 3 b4c4lh4u" but XORed with (key+i) for pwd[i]
const SECRET: XoredLiteral = XoredLiteral::from_xored(0xc0, b"\xb0\xa0\xb1\xb7\xa1\xe4\xb5\xe7\xac\xfa\xea\xa5\xad\xfa\xfa\xef\xe3\xf1\xb0\xe7\xb7\xe1\xba\xbf\xec\xac");

/// The decryption process XORes bytes from an input blob with an OTP
/// The OTP is obtained by using the CTR mode of operation:
///  IV + counter (on e.g. 32 bits) is hashed with e.g. SHA-256
///  which gives a stream of any length.
#[inline(never)]
fn decrypt(pwd: &[u8]) -> Option<String> {
    // Not compatible with windows, because of the relative path in the macro,
    //  but I don't expect we would have to build under windows
    let encrypted = include_bytes!("../encrypted_bugreport");
    let prefix_len: usize = 1024;

    // Will produce the stream of decrypted bytes
    let iter_decrypted = encrypted.iter().zip(otp_iter(pwd)).map(|(e, k)| e ^ k);
    // Skip the prefix noise
    let mut iter_decrypted = iter_decrypted.skip(prefix_len);
    // Read the length as a little endian u16
    let bytes_size: [u8; 2] = [iter_decrypted.next()?, iter_decrypted.next()?];
    //let bytes_size: [u8; 2] = iter_decrypted
    //    .take(2)
    //    .collect::<Vec<u8>>()
    //    .try_into()
    //    .ok()?;
    let text_len = u16::from_le_bytes(bytes_size);

    // Don't decrypt padding, but if there was an error, we risk having decoded too much,
    //  so check it afterwards
    let clear_bytes = iter_decrypted.take(text_len.into()).collect::<Vec<u8>>();

    // If we didn't decoded successfully, probably a pb with the password
    let clear_text = str::from_utf8(clear_bytes.as_slice()).ok()?;

    // Finally check the length
    if clear_text.len() == text_len.into() {
        Some(String::from(clear_text))
    } else {
        None
    }
}

use sha2::{Digest, Sha256};

/// Iterates over the bytes of the OTPad
/// This is slightly unreadable, but hey...
fn otp_iter(iv: &[u8]) -> impl Iterator<Item = u8> {
    // Chain together iterators on successive blocks
    (0..).flat_map(|i: u32| {
        // For each block, we merge the IV and the counter serialized as a big endian u32
        let mut v: Vec<u8> = Vec::from(iv.to_vec());
        v.extend(i.to_be_bytes());
        // Then iterate from the digest
        Sha256::digest(v.as_slice())
    })
}

#[cfg(test)]
mod tests_otp {
    use super::*;
    use hex_literal::hex;

    const TV: &[u8] = &hex!("
        460b8445f8f8309b6e7f42cedea67fa9c9c3dc434e441efff92dd63487a78ac25f16f4b74c281d990da9b4ecb3d7cc3c853c23876c092f9ce335d701dc1c0164
        141e8fc2f247bb1eb5c966ec9740c11d2bc34db11efae6903f34cbf24c4a3888816976e0601746f0ce05e22728c62f5bee4dfb354d904689e4677de4d858ffd3
        f92c0c4482352f035bd0d09a198dff70a923e4a8798031c741bed0eb0e48b7db3ee8faa61b1df19276849f897461bfa92dc7b52c4d80dcac83224e2d45346725
        5494d9cc45c5a10421c1b4129b756ea02b05186e297d101ff9d7b35eec040e925b82b05405a1fd79101bb7f74850af8340325c83f4b8fb98059f54a31922a655
    ");

    #[test]
    fn check_bytestream() {
        // Test vector only valid for this IV
        let iv = hex!("377385da04e14eb144193306");
        for (i, (expected, c)) in TV.iter().zip(otp_iter(iv.as_slice())).enumerate() {
            assert_eq!(*expected, c, "at index {i}");
        }
    }
}
