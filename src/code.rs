#[allow(dead_code)]

/// # Contians all return code for ftp response
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod code{
    pub type Code = &'static str;

    /// # welcome code 
    /// 
    /// code value : **220**
    /// 
    /// Associated message :
    /// - WELCOM_M
    pub const WELCOME_C : Code = "220";
    

    ///# session close code
    /// 
    /// code value : **221**
    /// 
    /// Associated message : 
    /// - BYE_M
    pub const BYE_C : Code = "221";
    
    ///# specify password code
    /// 
    /// code value : **331**
    /// 
    /// Associated message : 
    /// - SPEC_PASSWORD_M
    pub const SPEC_PASSWORD_C : Code = "331";
    
    ///# session open code
    /// 
    /// code value : **230**
    /// 
    /// Associated message : 
    /// - SESSION_OPEN_M
    pub const SESSION_OPEN_C : Code = "230";

    ///# session not open code
    /// 
    /// code value : **530**
    /// 
    /// Associated message :
    /// - SESSION_NO_OPEN_M
    /// - AUTH_ERROR_M
    /// - ANO_ONLY_M
    pub const SESSION_NO_OPEN_C : Code = "530";

    ///# unvalaible syntaxe command code
    /// 
    /// code value : **500**
    /// 
    /// Associated message : 
    /// - UNVA_SYNTAX_COMMAND_M
    pub const UNVA_SYNTAX_COMMAND_C : Code = "500";
    
    //UNVA_SYNTAX_ARGS_M
    ///# unvalaible syntaxe arguments code
    /// 
    /// code value : **501**
    /// 
    /// Associated message : 
    /// - UNVA_SYNTAX_ARGS_M
    pub const UNVA_SYNTAX_ARGS_C : Code = "501";

    ///# unknow command code
    /// 
    /// code value : **202**
    /// 
    /// Associated message : 
    /// - UNKNOW_COMMAND_M
    pub const UNKNONW_COMMAND_C : Code = "202";

    ///# passiv mode activate code
    /// 
    /// code value : **227**
    /// 
    /// Associated message : 
    /// - PASSIF_MODE_M
    pub const PASSIF_MODE_C : Code = "227";

    ///# bad command sequence code
    /// 
    /// code value : **503**
    /// 
    /// Associated message : 
    /// - BAD_COM_SEQ_M
    pub const BAD_COM_SEQ_C : Code = "503";

    ///# service unvalaible code
    /// 
    /// code value : **421**
    /// 
    /// Associated message : 
    /// - SERVICE_UNVA_M
    pub const SERVICE_UNVA_C : Code = "421";

    ///# data are comming code
    /// 
    /// code value : **150**
    /// 
    /// Associated message : 
    /// - DATA_COME_M
    pub const DATA_COME_C : Code = "150";

    ///# data are send code
    /// 
    /// code value : **226**
    /// 
    /// Associated message : 
    /// - DATA_SEND_M
    pub const DATA_SEND_C : Code = "226";
}