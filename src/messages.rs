#[allow(dead_code)]
/// # Contains all messages send by server
pub mod messages{

    pub type Message = &'static str;

    pub const WELCOM_MES : Message = "FTP Server (Axolotl FTP)";
    pub const BYE_MES : Message = "Session close, bye bye!";
    pub const SPEC_PASSWORD : Message = "Please specify the password";
    pub const AUTH_ERROR : Message = "Please login with USER and PASS";
    pub const ANO_ONLY : Message = "This FTP server is anonymous only";
    pub const UNVA_SYNTAX_COMMAND : Message = "Unvalid command error";
    pub const UNVA_SYNTAX_ARGS : Message = "Unvalid arguments error";
    pub const SESSION_OPEN_MES : Message = "Session open";
    pub const UNKNOWN_COMMAND_MES : Message = "Unknow command error";
}