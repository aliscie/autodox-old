use candid::CandidType;

#[derive(CandidType)]
pub enum AddChildContentResponse{
    Success,
    UserNotRegisted,
    Unauthorized,
    FileDoesNotExist,
}


#[derive(CandidType)]
pub enum AddViewerResponse{
    Success,
    InvalidUser,
    Unauthorized,
    AlreadyExist,
    FileDoesNotExist,
    UserNotRegisted,
}

#[derive(CandidType)]
pub enum ChangeFileModeResponse{
    Unauthorized,
    UserNotRegisted,
    FileDoesNotExist,
    Success,
}

#[derive(CandidType)]
pub enum CreateFileResponse{
    Success,
    UserNotRegisted,
}

#[derive(CandidType)]
pub enum DeleteFileResponse{
    Success,
    UserNotRegisted,
    FileDoesNotExist,
    Unauthorized,
}


#[derive(CandidType)]
pub enum RegisterResponse{
    Success{user_name: String},
    AlreadyRegistered{user_name: String},
    UserNameAlreadyInUse,
}


#[derive(CandidType)]
pub enum RemoveViewerResponse{
    Success,
    InvalidUser,
    Unauthorized,
    ViewerNotFound,
    FileDoesNotExist,
    UserNotRegisted,
}



#[derive(CandidType)]
pub enum UpdateFileResponse{
    Success,
    UserNotRegisted,
    FileDoesNotExist,
    Unauthorized,
}
