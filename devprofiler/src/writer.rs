use std::fs::File;
use std::io::Error;

use std::io::BufWriter;
use flate2::Compression;


pub struct OutputWriter {
    writer: GzEncoder<BufWriter<File>>,
}

impl OutputWriter {
    pub fn new() -> Result<OutputWriter, Error>{
        let file = File::create("devprofile.jsonl.gz")?;
        let bufw = BufWriter::new(file);
        let gze = GzEncoder::new(bufw, Compression::default());
          
        })
    }

        writeln!(self.writer, "Some prefix: {}", line.to_string())
        writeln!(self.writer, "{} ::  some suffix", line
        )
    }
            match self.create_io_file_writer() {
                Ok(io_writer) => {self.iowriter = io_writer;},
                Err(error) => { return Err(error); },
            }
            
        }
        let writer_borrow = self.iowriter.as_mut().expect("Checked, is some");
        writeln!(writer_borrow, "{}", line.to_string())
    }

    pub fn finish(&mut self) -> Result<(), Error>{
        if self.iowriter.is_some() {
            let writer_borrow = self.iowriter.as_mut().expect("Checked, is some");
            writer_borrow.flush()?;
        }
        self.writer.try_finish()
    }

    fn create_io_file_writer(&self) -> Result<Option<BufWriter<File>>, Error>{
        let iofile = File::create("io_errors.txt")?;
        Ok(Some(BufWriter::new(iofile)))
    }
}


        let repo_name = repo.name();
        log::debug!("[setup_self_host_user_repos_github]/repo_name: {:?}", &repo_name.to_string());
        if list.contains(&repo_name.as_str()){
            log::debug!("[setup_self_host_user_repos_github]/repo_name inside for loop: {:?}", &repo_name.to_string());
            let mut repo_copy = repo.clone();
            clone_git_repo(&mut repo_copy, access_token, &repo_provider).await;
            let repo_name = repo.name();
            let repo_owner = repo.owner();
            repo_owner_map
                
                .push(repo_name.to_string());
            
            );
            log::debug!("[setup_self_host_user_repos_github] Repo name = {:?}", repo_name);
            process_webhooks(repo_owner.to_string(), repo_name.to_string(), access_token.to_string())
                .await;
    
            let repo_name_async = repo_name.clone();
            let repo_owner_async = repo_owner.clone();
            let access_token_async = access_token.to_string().clone();
            task::spawn(async move {
                process_prs(&repo_owner_async, &repo_name_async, &access_token_async).await;
            });
        }
    }
