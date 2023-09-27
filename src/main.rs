
mod module_actif;


fn main() {
    let (module_actif_handler, module_actif_sender) = module_actif::start();
    module_actif::ask_action(&module_actif_sender);
    module_actif::stop(module_actif_handler, &module_actif_sender);
}











//utilisation mutex :

//let counter = Arc::new(Mutex::new(0));
//let counter = Arc::clone(&counter);
//*data += 1; // Modify the shared data
//*data += 1; // Modify the shared data