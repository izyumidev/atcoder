use {{crate_name}}::a::process;

fn main() {
	let file = include_str!("../../b.txt");
	process(file);
}
