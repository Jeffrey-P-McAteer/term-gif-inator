
use std::io::{Read, Write};


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let term_exe = get_parent_terminal_executable()?;

  println!("term_exe = {:?}", term_exe);

  let mut p = subprocess::Popen::create(&[ term_exe ], subprocess::PopenConfig {
      stdout: subprocess::Redirection::Pipe,
      stderr: subprocess::Redirection::Pipe,
      stdin: subprocess::Redirection::Pipe,
      ..Default::default()
  })?;

  // Read all stdout & stderr one character at a time, forwarding to stderr + stdout
  loop {
    // Read any available stdin bytes to communicate in
    let mut stdin_bytes_buff: [u8; 1024] = [0u8; 1024];
    let num_bytes_read = std::io::stdin().read(&mut stdin_bytes_buff)?;

    let (out, err) = p.communicate_bytes( Some(&stdin_bytes_buff[0..num_bytes_read]) )?;
    if let Some(stdout_bytes) = out {
      std::io::stdout().write_all(&stdout_bytes)?;
      std::io::stdout().flush()?;
    }
    if let Some(stderr_bytes) = err {
      std::io::stderr().write_all(&stderr_bytes)?;
      std::io::stderr().flush()?;
    }

  }



  Ok(())
}


fn get_parent_terminal_executable() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
  #[cfg(unix)]
  {
    let ppid = std::os::unix::process::parent_id();
    let parent_proc = procfs::process::Process::new(ppid as i32)?;
    return Ok(parent_proc.exe()?);
  }
  #[cfg(windows)]
  {
    compile_error!("get_parent_terminal_executable must be extended to support your windows system!");
  }
  #[cfg(not(any(unix, windows)))]
  compile_error!("get_parent_terminal_executable must be extended to support your non-windows, non-linux system!");
}



