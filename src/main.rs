use homebot::homebot_aux_funcs::*;
use homebot::loggin;

const CFGFILE: &str = "cfg.yaml";

#[cfg(any(feature = "test", feature = "live"))]
use homebot::env::*;
#[cfg(feature = "sim")]
use homebot::sim_env::*;

fn main() {
    println!("Running...");
    match load(CFGFILE) {
        Ok(mut cfg) => {
            #[cfg(feature = "test")]
            cfg.insert("MODE".to_string(), "Code Testing".to_string());
            #[cfg(feature = "sim")]
            cfg.insert("MODE".to_string(), "Webots Simulation".to_string());
            #[cfg(feature = "live")]
            cfg.insert("MODE".to_string(), "real Hardware Run".to_string());
            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
            log.info(&format!("- Mode: {}", cfg["MODE"]));
            match run(log.clone(), cfg.clone()) {
                Ok(()) => (),
                Err(es) => {
                    log.err(&format!("ERROR while on {}: {:#?}", cfg["MODE"], es));
                }
            };
        }
        Err(e) => {
            let log = loggin::Log::init("DEBUG".to_string());
            log.err(&format!("ERROR Reading YAML: {:#?}", e));
        }
    };
    println!("Bye!");
}

//#[cfg(feature = "sim")]
//fn main() {
//    match load(CFGFILE) {
//        Ok(cfg) => {
//            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
//            log.info(&format!("- Mode: Webots Simulation"));
//            match run(log.clone(), cfg) {
//                Ok(()) => (),
//                Err(es) => {
//                    log.err(&format!("ERROR running simulation: {:#?}", es));
//                }
//            };
//        }
//        Err(e) => {
//            let log = loggin::Log::init("DEBUG".to_string());
//            log.err(&format!("ERROR Reading YAML: {:#?}", e));
//        }
//    };
//}
//
//#[cfg(feature = "test")]
//fn main() {
//    match load(CFGFILE) {
//        Ok(cfg) => {
//            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
//            check_cfg(cfg.clone(), CFGFILE, log.clone());
//            log.info(&format!("- Mode: Code Tests"));
//            match run(log.clone(), cfg) {
//                Ok(()) => (),
//                Err(es) => {
//                    log.err(&format!("ERROR running tests: {:#?}", es));
//                }
//            };
//        }
//        Err(e) => {
//            let log = loggin::Log::init("DEBUG".to_string());
//            log.err(&format!("ERROR Reading YAML: {:#?}", e));
//        }
//    };
//}
//
//#[cfg(feature = "live")]
//fn main() {
//    match load(CFGFILE) {
//        Ok(cfg) => {
//            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
//            check_cfg(cfg.clone(), CFGFILE, log.clone());
//            log.info(&format!("- Mode: Hardware Live"));
//            //match run(log.clone(), cfg) {
//            //    Ok(()) => (),
//            //    Err(es) => {
//            //        log.err(&format!("ERROR running live: {:#?}", es));
//            //    }
//            //};
//        }
//        Err(e) => {
//            let log = loggin::Log::init("DEBUG".to_string());
//            log.err(&format!("ERROR Reading YAML: {:#?}", e));
//        }
//    };
//}
