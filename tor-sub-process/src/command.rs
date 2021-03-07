#[derive(Clone)]
pub struct Command {
    program: String,
    no_window_support: bool,
    tor_rc: String,
}

impl Command {
    /// Constructs a new `Command` for launching Tor at the path `program`.
    ///
    /// **Windows Support:**
    ///
    /// The `no_window_support` flag can be used to enable support for Tor
    /// being created with `CREATE_NO_WINDOW` flag on Windows platform.
    ///
    /// See:
    /// * [MSDN](https://docs.microsoft.com/en-gb/windows/win32/procthread/process-creation-flags?redirectedfrom=MSDN#CREATE_NO_WINDOW)
    /// * [First commit](https://github.com/torproject/tor/commit/b60049544143e8569e491dd30541d28127bfdb22)
    /// * [Latest commit](https://github.com/torproject/tor/blob/tor-0.4.5.6/src/lib/process/process_win32.c#L208-L219)
    pub fn new(program: &str, tor_rc: &str, no_window_support: bool) -> Self {
        Self {
            program: program.to_string(),
            no_window_support,
            tor_rc: tor_rc.to_string(),
        }
    }

    #[cfg(target_family = "unix")]
    pub fn create(&self) -> tokio::process::Command {
        /* Runs program in a new session to avoid spawned child process to receive double SIGINT
         *   * one SIGINT from current program during graceful shutdown
         *   * one SIGINT from terminal when CTRL+C sends SIGINT to all process in same PGID
         */

        let args = &self.program.split(" ").collect::<Vec<_>>();
        let (c, a) = args.split_first().unwrap();

        let mut command = tokio::process::Command::new("setsid");
        command.arg(c).args(a).args(&["-f", &self.tor_rc]);
        command
    }

    #[cfg(target_family = "windows")]
    pub fn create(&self) -> tokio::process::Command {
        if self.no_window_support {
            /* Pipes output of program to `more` in order to attach the windowless program to console.
             * Runs program using powershell in order for program to receive signals.
             */
            let mut command = tokio::process::Command::new("powershell");
            command.arg(format!("{} | more", &self.program));
            command
        } else {
            let args = &self.program.split(" ").collect::<Vec<_>>();
            let (c, a) = args.split_first().unwrap();

            let mut command = tokio::process::Command::new(&self.program);
            command.arg(c).args(a).args(&["-f", &self.tor_rc]);
            command
        }
    }
}
