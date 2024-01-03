use {{crate_name}}::a::process;

fn main() {
	let file = include_str!("../../a.txt");
	process(file);
}
