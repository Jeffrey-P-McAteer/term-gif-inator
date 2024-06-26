
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .thread_stack_size(3 * 1024 * 1024)
    .enable_all()
    .build()?;

  rt.block_on(async {
    if let Err(e) = async_main().await {
      eprintln!("Error: {:?}", e);
    }
  });

  Ok(())
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
  let term_exe = get_parent_terminal_executable()?;
  println!("term_exe = {:?}", term_exe);

  let mut child = tokio::process::Command::new(term_exe)
    .spawn()?;



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



