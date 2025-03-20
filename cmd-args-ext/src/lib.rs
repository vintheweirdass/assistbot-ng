// cmd-args-ext

use serenity::all::{CommandInteraction, CommandOptionType, CreateCommand, ResolvedOption, ResolvedValue};

#[derive(Debug)]
pub enum CommandError {
    Default(String),
    Argument(String, String),
}
pub trait EnumArgsExt: Default {
    fn to_alias(&self)->String;
}
pub trait CommandArgsExt: Default {
    fn add_to_command(command: CreateCommand) -> CreateCommand;
    fn from_options(options: &[ResolvedOption]) -> Result<Self, CommandError>;
    fn from_command(command: &CommandInteraction) -> Result<Self, CommandError>;
}

pub trait CommandOptionTypeExt: Sized {
    fn get_option_type() -> CommandOptionType;
    fn from_option(option: Option<&ResolvedOption>) -> Result<Self, CommandError>;
}

impl CommandOptionTypeExt for String {
    fn get_option_type() -> CommandOptionType {
        CommandOptionType::String
    }

    fn from_option(option: Option<&ResolvedOption>) -> Result<Self, CommandError> {
        if let Some(option) = option {
            if let ResolvedValue::String(value) = &option.value {
                Ok(value.to_string())
            } else {
                Err(CommandError::Argument(option.name.to_string(), "Expected string value".to_string()))
            }
        } else {
            Err(CommandError::Default("Expected string value".to_string()))
        }
    }
}

impl CommandOptionTypeExt for f64 {
    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Number
    }

    fn from_option(option: Option<&ResolvedOption>) -> Result<Self, CommandError> {
        if let Some(option) = option {
            if let ResolvedValue::Number(value) = &option.value {
                Ok(*value)
            } else {
                Err(CommandError::Argument(option.name.to_string(), "Expected number value".to_string()))
            }
        } else {
            Err(CommandError::Default("Expected number value".to_string()))
        }
    }
}

impl CommandOptionTypeExt for u64 {
    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Number
    }

    fn from_option(option: Option<&ResolvedOption>) -> Result<Self, CommandError> {
        if let Some(option) = option {
            if let ResolvedValue::Number(value) = &option.value {
                let rounded_value = value.round();
            
                if rounded_value < 0.0 {
                    return Err(CommandError::Argument(option.name.to_string(), "Expected non-negative interger value".to_string()));
                }
            
                if rounded_value > u64::MAX as f64 {
                    return Err(CommandError::Argument(option.name.to_string(), "The number is too large".to_string()))
                }
                return Ok(rounded_value as u64);
            } else {
                Err(CommandError::Argument(option.name.to_string(), "Expected non-negative interger value".to_string()))
            }
        } else {
            Err(CommandError::Default("Expected non-negative interger value".to_string()))
        }
    }
}

impl CommandOptionTypeExt for u32 {
    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Number
    }

    fn from_option(option: Option<&ResolvedOption>) -> Result<Self, CommandError> {
        if let Some(option) = option {
            if let ResolvedValue::Number(value) = &option.value {
                let rounded_value = value.round();
            
                if rounded_value < 0.0 {
                    return Err(CommandError::Argument(option.name.to_string(), "Expected non-negative interger value".to_string()));
                }
            
                if rounded_value > u32::MAX as f64 {
                    return Err(CommandError::Argument(option.name.to_string(), "The number is too large".to_string()))
                }
                return Ok(rounded_value as u32);
            } else {
                Err(CommandError::Argument(option.name.to_string(), "Expected non-negative interger value".to_string()))
            }
        } else {
            Err(CommandError::Default("Expected non-negative interger value".to_string()))
        }
    }
}


impl CommandOptionTypeExt for i64 {
    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Number
    }

    fn from_option(option: Option<&ResolvedOption>) -> Result<Self, CommandError> {
        if let Some(option) = option {
            if let ResolvedValue::Number(value) = &option.value {
                let rounded_value = value.round();
            
                if rounded_value < i64::MIN as f64 {
                    return Err(CommandError::Argument(option.name.to_string(), "The number is too small".to_string()));
                }
            
                if rounded_value > i64::MAX as f64 {
                    return Err(CommandError::Argument(option.name.to_string(), "The number is too large".to_string()))
                }
                return Ok(rounded_value as i64);
            } else {
                Err(CommandError::Argument(option.name.to_string(), "Expected interger value".to_string()))
            }
        } else {
            Err(CommandError::Default("Expected interger value".to_string()))
        }
    }
}


impl CommandOptionTypeExt for i32 {
    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Number
    }

    fn from_option(option: Option<&ResolvedOption>) -> Result<Self, CommandError> {
        if let Some(option) = option {
            if let ResolvedValue::Number(value) = &option.value {
                let rounded_value = value.round();
            
                if rounded_value < i32::MIN as f64 {
                    return Err(CommandError::Argument(option.name.to_string(), "The number is too small".to_string()));
                }
            
                if rounded_value > i32::MAX as f64 {
                    return Err(CommandError::Argument(option.name.to_string(), "The number is too large".to_string()))
                }
                return Ok(rounded_value as i32);
            } else {
                Err(CommandError::Argument(option.name.to_string(), "Expected interger value".to_string()))
            }
        } else {
            Err(CommandError::Default("Expected interger value".to_string()))
        }
    }
}


impl CommandOptionTypeExt for f32 {
    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Number
    }

    fn from_option(option: Option<&ResolvedOption>) -> Result<Self, CommandError> {
        if let Some(option) = option {
            if let ResolvedValue::Number(v) = &option.value {    
                let value = *v;
                if value < f32::MIN as f64 {
                    return Err(CommandError::Argument(option.name.to_string(), "The number is too small".to_string()));
                }
            
                if value > f32::MAX as f64 {
                    return Err(CommandError::Argument(option.name.to_string(), "The number is too large".to_string()))
                }
                return Ok(value as f32);
            } else {
                Err(CommandError::Argument(option.name.to_string(), "Expected number value".to_string()))
            }
        } else {
            Err(CommandError::Default("Expected number value".to_string()))
        }
    }
}



impl CommandOptionTypeExt for bool {
    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Boolean
    }

    fn from_option(option: Option<&ResolvedOption>) -> Result<Self, CommandError> {
        if let Some(option) = option {
            if let ResolvedValue::Boolean(value) = &option.value {
                Ok(*value)
            } else {
                Err(CommandError::Argument(option.name.to_string(), "Expected boolean value".to_string()))
            }
        } else {
            Err(CommandError::Default("Expected boolean value".to_string()))
        }
    }
}

impl<T> CommandOptionTypeExt for Option<T> where T: CommandOptionTypeExt {
    fn get_option_type() -> CommandOptionType {
        T::get_option_type()
    }

    fn from_option(option: Option<&ResolvedOption>) -> Result<Self, CommandError> {
        // If the option is None, return Ok(None) since this is valid for Option<T>
        if option.is_none() {
            return Ok(None);
        }
        
        // Otherwise, try to parse the value using T's implementation
        match T::from_option(option) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None) // Or you could propagate the error if preferred
        }
    }
}


// impl<T> CommandOptionTypeExt for Option<T> where T: CommandOptionTypeExt {
//     fn get_option_type() -> CommandOptionType {
//         CommandOptionType::
//     }

//     fn from_option(&self, option: Option<&ResolvedOption>) -> Result<Self, CommandError> {
//         if let Some(_) = option {
//             return Ok(_);
//         } else {
            
//         }
//     }
// }

pub trait CreateCommandExt {
    fn add_args<T: CommandArgsExt>(self) -> CreateCommand;
}

impl CreateCommandExt for CreateCommand {
    fn add_args<T: CommandArgsExt>(self) -> CreateCommand {
        T::add_to_command(self)
    }
}

pub fn extract_string(value: &ResolvedValue) -> Result<String, CommandError> {
    match value {
        ResolvedValue::String(s) => Ok(s.to_string()),
        _ => Err(CommandError::Default("Expected string value".to_string())),
    }
}