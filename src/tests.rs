use crate::{levels, plog};

#[cfg(test)]
fn plog_test() {

   plog::log(levels::LogLevel::Warn, "IM WARNING YOU!".to_string()); 
   plog::log(levels::LogLevel::Err, "IM ERRORING YOU!!!".to_string());
   plog::log(levels::LogLevel::Info, "IM INFORMING YOU!!!".to_string());
   plog::log(levels::LogLevel::Fatal, "IM KILLING YOU!!!".to_string()); 


}
