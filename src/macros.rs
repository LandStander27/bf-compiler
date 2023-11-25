#[macro_export]
macro_rules! print_error {
	($string:expr $(, $opt:expr)*) => {
		println!("{}{}", style("[ERR] ").red().bold(), format!($string, $($opt),*));
		std::process::exit(1);
	};
}

// #[macro_export]
// macro_rules! print_warning {
// 	($string:expr $(, $opt:expr)*) => {
// 		println!("{}{}", style("[WAR] ").yellow().bold().dim(), format!($string, $($opt),*));
// 	};
// }

#[macro_export]
macro_rules! print_info {
	($string:expr $(, $opt:expr)*) => {
		println!("{}{}", style("[INF] ").bold().dim(), format!($string, $($opt),*));
	};
}

// #[macro_export]
// macro_rules! print_step {
// 	($current:expr, $total:expr, $string:tt $(, $opt:expr)*) => {
// 		println!("{}{}", style(format!("[{}/{}] ", $current, $total)).bold().dim(), format!($string, $($opt),*));
// 	};
// }