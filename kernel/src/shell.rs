use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

const BS: u8 = 0x08;
const BEL: u8 = 0x07;
const LF: u8 = 0x0A;
const CR: u8 = 0x0D;
const DEL: u8 = 0x7F;

/// Parse an input command line and deal with bells etc.
/// Here I started to use vincenthouyi's repo and code for heavy inspiration
fn readline(buf: &mut [u8]) -> &str {
    let mut read = 0;
    loop {
        let b = CONSOLE.lock().read_byte();
        match b {
            BS | DEL if read > 0 => {
                read -= 1;
                kprint!("{}", BS as char);
                kprint!(" ");
                kprint!("{}", BS as char);
            }
            LF | CR => {
                kprintln!();
                break;
            }
            _ if read == buf.len() => {
                kprint!("{}", BEL as char);
            }
            byte @ b' '... b'~' => {
                buf[read] = byte;
                read += 1;
                kprint!("{}", byte as char);
            }
            _ => kprint!("{}", BEL as char),
        }
    }
    use std::str;
    return str::from_utf8(&buf[..read]).unwrap();
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    let mut buf = [0u8; 512];
    loop {
        kprint!("{}", prefix);
        let cmd = readline(&mut buf);
        match Command::parse(cmd, &mut [""; 64]) {
            Err(Error::TooManyArgs) => kprintln!("Too many arguments"),
            Err(Error::Empty) => { }
            Ok(cmd) => {
                match cmd.path() {
                    "echo" => kprintln!("{}", cmd.args[1]),
                    v => kprintln!("unknown command: {}", v),
                }
            }
        }
    }
}
