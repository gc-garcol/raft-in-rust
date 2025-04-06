use super::logging::init_logging;

pub struct InfraBootstrap {
}

impl InfraBootstrap {
    pub fn bootstrap() {
        init_logging();
    }
}