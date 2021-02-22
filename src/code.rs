#[allow(dead_code)]
///
/// # Contains all return code 
/// 
pub mod code{
    pub type Code = &'static str;

    //WELCOM_M
    pub const WELCOME_C : Code = "220";
    
    //BYE_M
    pub const BYE_C : Code = "221";
    
    //SPEC_PASSWORD_M
    pub const SPEC_PASSWORD_C : Code = "331";
    
    //SESSION_OPEN_C
    pub const SESSION_OPEN_C : Code = "230";

    //SESSION_NO_OPEN_M
    //AUTH_ERROR_M
    //ANO_ONLY_M
    pub const SESSION_NO_OPEN_C : Code = "530";

    //UNVA SYNTAC_COMMAND_M
    pub const UNVA_SYNTAX_COMMAND_C : Code = "500";
    
    //UNVA_SYNTAX_ARGS_M
    pub const UNVA_SYNTAX_ARGS_C : Code = "501";

    //UNKNOW_COMMAND_M
    pub const UNKNONW_COMMAND_C : Code = "202";

    //PASSIV_MODE_M
    pub const PASSIF_MODE_C : Code = "227";

    //BAD_COM_SEQ_M
    pub const BAD_COM_SEQ_C : Code = "503";

    //SERVUCE_UNVA_M
    pub const SERVICE_UNVA_C : Code = "421";

    //DATA_COME_M
    pub const DATA_COME_C : Code = "150";

    //DATA_SEND_M
    pub const DATA_SEND_C : Code = "226";
}