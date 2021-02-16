#[allow(dead_code)]
///
/// # Contains all return code 
/// 
pub mod code{
    pub type Code = &'static str;

    pub const WELCOME : Code = "220";
    pub const BYE : Code = "221";
    pub const NEED_PASSWORD : Code = "331";
    pub const SESSION_NO_OPEN : Code = "530";
    pub const SESSION_OPEN : Code = "230";
    pub const SYNTAX_COMMAND_ERROR : Code = "500";
    pub const SYNTAX_ARGS_ERROR : Code = "501";
    pub const UNKNONW_COMMAND_C : Code = "202";
    pub const PASSIF_MODE_C : Code = "227";
    pub const BAD_COM_SEQ_C : Code = "503";
    pub const SERVICE_UNA_C : Code = "421";
}