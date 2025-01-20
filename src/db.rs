use std::fs::{self, OpenOptions};
use std::io::{Read, Result, Write};
use std::os::unix::fs::FileExt;
use teloxide::prelude::*;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    Credentials,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct Data {
    pub messages: Vec<ChatCompletionMessage>,
}

pub fn check_dir() -> Result<()> {
    let dir_name = format!("conversations");
    if !fs::exists(dir_name.clone())? {
        fs::create_dir(dir_name)?;
    }
    Ok(())
}

pub fn push_converstaion(chat_id: ChatId, chat: ChatCompletionMessage) -> Result<()> {
    let file_name = format!("conversations/{}", chat_id.0);
    check_dir()?;
    let mut f = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(file_name)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let mut data: Data = serde_json::from_str(&buf).unwrap_or_default();
    data.messages.push(chat);
    f.write_all_at(serde_json::to_string(&data).unwrap().as_bytes(), 0)?;
    f.sync_all()?;
    Ok(())
}

pub fn reset_converstaion(chat_id: ChatId) -> Result<()> {
    let file_name = format!("conversations/{}", chat_id.0);
    check_dir()?;
    fs::remove_file(file_name)?;
    Ok(())
}

pub fn get_data(chat_id: ChatId) -> Result<Data> {
    let file_name = format!("conversations/{}", chat_id.0);
    check_dir().unwrap();
    let mut f = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(file_name).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let data: Data = serde_json::from_str(&buf).unwrap_or_default();
    f.write_all_at(serde_json::to_string(&data).unwrap().as_bytes(), 0)?;
    f.sync_all()?;
    Ok(data)
}