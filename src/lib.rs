extern crate failure;
extern crate ron;
extern crate serde;
use crate::KvError::{FileOpenError, KeyNotExists, WriteError};
use failure::Fail;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

pub struct KvStore {
    log: File,
    log_pointer: HashMap<String, usize>,
}

#[derive(Fail, Debug)]
pub enum KvError {
    #[fail(display = "key[{}] is not exists", key)]
    KeyNotExists { key: String },
    #[fail(display = "open file[{}] failed", path)]
    FileOpenError { path: String },
    #[fail(display = "failed to write")]
    WriteError,
}

pub type Result<T> = std::result::Result<T, KvError>;

impl KvStore {
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let idx = self.log.seek(SeekFrom::End(0)).unwrap();
        let ret = self.write(Command::Set {
            key: key.clone(),
            value,
        });
        self.log_pointer.insert(key, idx as usize);
        ret
    }

    fn write(&mut self, cmd: Command) -> Result<()> {
        let serialized = ron::ser::to_string(&cmd).unwrap() + "\n";
        match self.log.write(serialized.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(WriteError {}),
        }
    }

    fn initialize_if_not(&mut self) {
        if self.log_pointer.is_empty() {
            self.log.seek(SeekFrom::Start(0)).unwrap();
            let mut buf = String::new();
            self.log.read_to_string(&mut buf).unwrap();
            if buf.is_empty() {
                return;
            }
            buf[..buf.len() - 1]
                .split('\n')
                .map(|line| (line.len() + 1, ron::de::from_str::<Command>(&line).unwrap()))
                .fold(
                    (&mut self.log_pointer, 0),
                    |(log_pointer, idx), (len, cmd)| {
                        match cmd {
                            Command::Set { key, value: _value } => {
                                log_pointer.insert(key, idx);
                            }
                            Command::Remove { key } => {
                                log_pointer.remove(&key);
                            }
                        }
                        (log_pointer, idx + len)
                    },
                );
        }
    }

    fn read(&mut self, idx: usize) -> String {
        self.log.seek(SeekFrom::Start(idx as u64)).unwrap();
        let mut line = String::new();
        BufReader::new(self.log.try_clone().unwrap())
            .read_line(&mut line)
            .unwrap();
        match ron::de::from_str::<Command>(&line).unwrap() {
            Command::Set {  value , ..} => value,
            Command::Remove { .. } => panic!("index {} locates Command::Remove", idx),
        }
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.initialize_if_not();
        match self.log_pointer.get(&key).cloned() {
            Some(idx) => Ok(Some(self.read(idx))),
            None => Ok(None),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.initialize_if_not();
        match self.log_pointer.get(&key) {
            Some(_) => {
                let ret = self.write(Command::Remove { key: key.clone() });
                self.log_pointer.remove(&key);
                ret
            }
            None => Err(KeyNotExists { key }),
        }
    }

    pub fn open(path_buf: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path_buf = path_buf.into();
        path_buf.push("kvs.db");
        match OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path_buf)
        {
            Err(_) => Err(FileOpenError {
                path: path_buf.to_str().unwrap().into(),
            }),
            Ok(log) => Ok(KvStore {
                log,
                log_pointer: HashMap::new(),
            }),
        }
    }
}
