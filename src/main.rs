use brainf::BrainFuckVM;

fn main() {
    let content = String::from("++++[-]");
    let mut vm = BrainFuckVM::new(content);
    vm.exec();
}
