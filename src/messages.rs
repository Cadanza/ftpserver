#[allow(dead_code)]
/// # Contains all messages send by server
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod messages{

    pub type Message = &'static str;

    /// # welcome message
    /// 
    /// message value : **FTP Server (Axolotl FTP)**
    /// 
    /// Associated Code :
    /// - WELCOM_C
    pub const WELCOM_M : Message = "FTP Server (Axolotl FTP)";

    /// # session close message
    /// 
    /// message value : **Session close, bye bye!**
    /// 
    /// Associated Code :
    /// - BYE_C
    pub const BYE_M : Message = "Session close, bye bye!";

    /// # specify password message
    /// 
    /// message value : **Please specify the password**
    /// 
    /// Associated Code :
    /// - SPEC_PASSWORD_C
    pub const SPEC_PASSWORD_M : Message = "Please specify the password";
    
    /// # session open message
    /// 
    /// message value : **Session Open**
    /// 
    /// Associated Code :
    /// - SESSION_OPEN_C
    pub const SESSION_OPEN_M : Message = "Session open";
    
    //SESSION_NO_OPEN_C
    /// # session no open message
    /// 
    /// message value : **Session no open**
    /// 
    /// Associated Code :
    /// - SESSION_NO_OPEN_C
    pub const SESSION_NO_OPEN_M : Message = "Session no open";

    /// # authentification error message
    /// 
    /// message value : **Please login with USER and PASS**
    /// 
    /// Associated Code :
    /// - SESSION_NO_OPEN_C
    pub const AUTH_ERROR_M : Message = "Please login with USER and PASS";

    /// # anonymous only message
    /// 
    /// message value : **This FTP server is anonymous only**
    /// 
    /// Associated Code :
    /// - SESSION_NO_OPEN_C
    pub const ANO_ONLY_M : Message = "This FTP server is anonymous only";
    
    /// # unvalaible syntaxe command message
    /// 
    /// message value : **Unvalid command error**
    /// 
    /// Associated Code :
    /// - UNVA_SYNTAX_COMMAND_C
    pub const UNVA_SYNTAX_COMMAND_M : Message = "Unvalid command error";
    
    /// # unvalaible syntaxe arguments message
    /// 
    /// message value : **Unvalid arguments error**
    /// 
    /// Associated Code :
    /// - UNVA_SYNTAX_ARGS_C
    pub const UNVA_SYNTAX_ARGS_M : Message = "Unvalid arguments error";
    
    /// # Unknow command message
    /// 
    /// message value : **Unknow command error**
    /// 
    /// Associated Code :
    /// - UNKNOW_COMMAND_C
    pub const UNKNOWN_COMMAND_M : Message = "Unknow command error";
    
    /// # passiv mod activate message
    /// 
    /// message value : **Enterring passive mod**
    /// 
    /// Associated Code :
    /// - PASSIV_MODE_C
    pub const PASSIF_MODE_M : Message = "Enterring passive mod";
    
    /// # bad command sequence message
    /// 
    /// message value : **Bad command sequences**
    /// 
    /// Associated Code :
    /// - BAD_COM_SEQ_C
    pub const BAD_COM_SEQ_M : Message = "Bad command sequences";
    
    /// # service unvalaible message
    /// 
    /// message value : **Service is not available**
    /// 
    /// Associated Code :
    /// - SERVICA_UNVA_C
    pub const SERVICE_UNVA_M : Message = "Service is not available";
    
    /// # data are comming message
    /// 
    /// message value : **Here commes the directoly listing**
    /// 
    /// Associated Code :
    /// - DATA_COME_C
    pub const DATA_COME_M : Message = "Here commes the directoly listing";
    
    /// # data are send message
    /// 
    /// message value : **Directory send Ok.**
    /// 
    /// Associated Code :
    /// - DATA_SEND_C
    pub const DATA_SEND_M : Message = "Directory send Ok.";

    ///# command handle is conclud message
    /// 
    /// message value : **Command conclud**
    /// 
    /// Associated code : 
    /// - CONCLUD_COMMAND_C
    pub const CONCLUD_COMMAND_M : Message = "Command conclud";

    ///# data stream not open message
    /// 
    /// message value : **No data tcp stream founded**
    /// 
    /// Associated code : 
    /// - DATA_STREAM_ERROR_C
    pub const DATA_STREAM_ERROR_M : Message = "No data tcp stream fouded";

    ///# file service finish message
    /// 
    /// message value : **Service file finish**
    /// 
    /// Associated code : 
    /// - FILE_SERVICE_FINISH_C
    pub const FILE_SERVICE_FINISH_M : Message = "Service file finish";

    ///# file not accessible message
    /// 
    /// message value : **File asked is not accessible**
    /// 
    /// Associated code : 
    /// - FILE_NOT_ACCESS_C
    pub const FILE_NOT_ACCESS_M : Message = "File asked is not accesssible";

    ///# path created code
    /// 
    /// message value : **Path correctly created**
    /// 
    /// Associated message : 
    /// - PATH_CREATED_C
    pub const PATH_CREATED_M : Message = "Path correctly created";

    ///# path removed code
    /// 
    /// message value : **Path correctly removed**
    /// 
    /// Associated message : 
    /// - PATH_REMOVED_C
    pub const PATH_REMOVED_M : Message = "Path correctly removed";
}