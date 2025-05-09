pub const API_VERSION: &str = "v1";
pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 100;

pub const DEFAULT_CACHE_TTL: u64 = 3600; 
pub const MAX_CACHE_TTL: u64 = 86400; 

pub const JWT_TOKEN_PREFIX: &str = "Bearer ";
pub const ACCESS_TOKEN_DURATION: i64 = 3600;
pub const REFRESH_TOKEN_DURATION: i64 = 2592000;   

pub const RATE_LIMIT_WINDOW: u64 = 60;  
pub const RATE_LIMIT_MAX_REQUESTS: u32 = 100;