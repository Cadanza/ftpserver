#[allow(dead_code)]
/// # Contains all messages send by server
pub mod messages{


    pub const WELCOM_MES : &str = "FTP Server (Axolotl FTP)";
    pub const BYE_MES : &str = "Session close, bye bye!";
    pub const SPEC_PASSWORD : &str = "Please specify the password";
    pub const AUTH_ERROR : &str = "Please login with USER and PASS";
    pub const ANO_ONLY : &str = "This FTP server is anonymous only";
    pub const UNVA_SYNTAX_COMMAND : &str = "Unvalid command error";
    pub const UNVA_SYNTAX_ARGS : &str = "Unvalid arguments error";
    pub const SESSION_OPEN_MES : &str = "Session open";
    pub const UNKNOWN_COMMAND_MES : &str = "Unknow command error";
    pub const PASSIF_MODE_M : &str = "Enterring passive mod ";
    pub const BAD_COM_SEQ_M : &str = "Bad command sequences";
    pub const SERVICE_UNA_M : &str = "Service is not available";
}