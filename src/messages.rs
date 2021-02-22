#[allow(dead_code)]
/// # Contains all messages send by server
pub mod messages{

    pub type Message = &'static str;

    //WELCOME_C
    pub const WELCOM_M : Message = "FTP Server (Axolotl FTP)";

    //BYE_C
    pub const BYE_M : Message = "Session close, bye bye!";

    //SPEC_PASSWORD_C
    pub const SPEC_PASSWORD_M : Message = "Please specify the password";
    
    //SESSION_OPEN_C
    pub const SESSION_OPEN_M : Message = "Session open";
    
    //SESSION_NO_OPEN_C
    pub const SESSION_NO_OPEN_M : Message = "Session no open";
    pub const AUTH_ERROR_M : Message = "Please login with USER and PASS";
    pub const ANO_ONLY_M : Message = "This FTP server is anonymous only";
    
    //UNVA_SYNTAX_COMMAND_C
    pub const UNVA_SYNTAX_COMMAND_M : Message = "Unvalid command error";
    
    //UNVA_SYNTAX_ARGS_C
    pub const UNVA_SYNTAX_ARGS_M : Message = "Unvalid arguments error";
    
    //UNKNOW_COMMAND_C
    pub const UNKNOWN_COMMAND_M : Message = "Unknow command error";
    
    //PASSIV_MODE_C
    pub const PASSIF_MODE_M : Message = "Enterring passive mod ";
    
    //BAD_COM_SEQ_C
    pub const BAD_COM_SEQ_M : Message = "Bad command sequences";
    
    //SERVICA_UNVA_C
    pub const SERVICE_UNVA_M : Message = "Service is not available";
    
    //DATA_COME_C
    pub const DATA_COME_M : Message = "Here commes the directoly listing";
    
    //DATA_SEND_C
    pub const DATA_SEND_M : Message = "Directory send Ok.";
}