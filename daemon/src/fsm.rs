use crate::captive::Captive;
use crate::configs::Config;
use crate::event::Event;
use crate::platform::NetworkManager;
use std::any::Any;
use std::boxed::Box;

#[cfg(target_os = "macos")]
use crate::platform::macos::MacOSNetworkManager;

#[cfg(target_os = "linux")]
use crate::platform::linux::LinuxNetworkManager;

struct Context {
    pub config: Config,
    pub nm: Box<dyn NetworkManager>,
    pub captive: Captive,
}

trait State: Any {
    fn on_enter(&mut self, _ctx: &mut Context) {}
    fn on_exit(&mut self, _ctx: &mut Context) {}
    fn handle(&mut self, _ctx: &mut Context) -> Option<Box<dyn State>>;
    fn name(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;
}

struct Idle;
struct AdapterOn;
struct OnLoginPage;
// struct Notify;

impl State for Idle {
    fn name(&self) -> &'static str {
        "Idle"
    }
    fn handle(&mut self, _ctx: &mut Context) -> Option<Box<dyn State>> {
        match _ctx.nm.is_adapater_on() {
            Ok(true) => return Some(Box::new(AdapterOn)),
            Ok(false) => return Some(Box::new(Idle)),
            _ => None,
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl State for AdapterOn {
    fn name(&self) -> &'static str {
        "Wifi On"
    }
    fn handle(&mut self, _ctx: &mut Context) -> Option<Box<dyn State>> {
        if _ctx.nm.internet_available(_ctx.config.timeouts) {
            return Some(Box::new(Idle));
        } else if _ctx.captive.probe() {
            return Some(Box::new(OnLoginPage));
        }
        return Some(Box::new(Idle));
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl State for OnLoginPage {
    fn name(&self) -> &'static str {
        "OnLoginPage"
    }

    fn handle(&mut self, _ctx: &mut Context) -> Option<Box<dyn State>> {
        match _ctx.captive.login(&_ctx.config.profile) {
            Event::Success => Some(Box::new(Idle)),
            Event::MaxConcurrent => Some(Box::new(Idle)),
            Event::WrongCreds => Some(Box::new(Idle)),
            Event::Unknown => Some(Box::new(Idle)),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Machine {
    _ctx: Context,
    state: Box<dyn State>,
}
impl Machine {
    pub fn new(config: Config) -> Self {
        #[cfg(target_os = "macos")]
        return Self {
            _ctx: Context {
                captive: Captive::new(config.timeouts),
                config: config,
                nm: Box::new(MacOSNetworkManager::new()),
            },
            state: Box::new(Idle),
        };
        #[cfg(target_os = "linux")]
        return Self {
            _ctx: Context {
                captive: Captive::new(config.timeouts),
                config: config,
                nm: Box::new(LinuxNetworkManager::new()),
            },
            state: Box::new(Idle),
        };
        #[cfg(not(target_os = "macos"))]
        compile_error!("No network manager implemented for this platform yet")
    }

    pub fn reset(&mut self) {
        self.state = Box::new(Idle);
        self.dispatch();
    }

    pub fn dispatch(&mut self) {
        // print!("{} -> ", self.state.name());
        if let Some(mut new_state) = self.state.handle(&mut self._ctx) {
            self.state.on_exit(&mut self._ctx);
            new_state.on_enter(&mut self._ctx);
            self.state = new_state;
            if self.state.as_any().is::<Idle>() {
                // println!("end;");
                return;
            }
            self.dispatch();
        }
    }
}
