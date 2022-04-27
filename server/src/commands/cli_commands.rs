use crate::format_percent_usage;
use crate::metrics_collector_controllers::database;
use crate::metrics_collector_controllers::collector;
use crate::metrics_collector_controllers::collector_utils;

pub fn display_database_info() {
    let process_info = database::get_current_metrics_from_db();

    println!("{0: <7} | {1: <43} | {2: <10} | {3: <7} | {4: <7} | {5: <7}","PID", "NAME", "MEMORY", "CPU", "DISK", "NETWORK");
    println!("________________________________________________________________________________________________");
    for p in process_info.unwrap() {
        println!("{0: <7} | {1: <43} | {2: <10} | {3: <7} | {4: <7} | {5: <7}", p.proc_id, p.proc_name, p.proc_mem, p.proc_cpu, p.proc_disk_usage, p.proc_net_usage);
    }
}

pub fn display_cpu_info() -> f32 {
    let process_info = database::get_current_metrics_from_db();

    // String to be parsed into a float.
    let mut percent_usage_string: String = "".to_owned();

    // Variable to store percents as floats.
    let mut percent_usage: f32 = 0.0;

    // Used to find sum of all usages.
    let mut total_usage: f32 = 0.0;

    println!("{0: <7} | {1: <43} |{2: <7}", "PID", "NAME", "CPU");
    println!("---------------------------------------------------------");
    for p in process_info.unwrap() {
        // Check whether this process' CPU usage has been gathered.
        if p.proc_cpu != "LOADING" {
            // Get the percent usage string.
            percent_usage_string = p.proc_cpu.replace("%", "");

            // Convert the percent usage string to a float.
            percent_usage = percent_usage_string.parse::<f32>().unwrap();

            // Add to the total CPU usage.
            total_usage += percent_usage;
        }

        // Print the info the user wants.
        println!("{0: <7} | {1: <43} |{2: <7}", p.proc_id, p.proc_name, p.proc_cpu);
    }
    // Output the total CPU allocation.
    println!("TOTAL CPU USAGE OVER LAST INTERVAL: {0: <7}", format_percent_usage(total_usage));

    // Send back the total_usage for testing purposes.
    return total_usage;
}

pub fn display_disk_info() -> f32 {
    let process_info = database::get_current_metrics_from_db();

    // String to be parsed into a float.
    let mut percent_usage_string: String = "".to_owned();

    // Variable to store percents as floats.
    let mut percent_usage: f32 = 0.0;

    // Used to find sum of all usages.
    let mut total_usage: f32 = 0.0;

    println!("{0: <7} | {1: <43} |{2: <7}", "PID", "NAME", "DISK");
    println!("---------------------------------------------------------");
    for p in process_info.unwrap() {
        // Get the percent usage string.
        percent_usage_string = p.proc_disk_usage.replace("%", "");

        // Convert the percent usage string to a float.
        percent_usage = percent_usage_string.parse::<f32>().unwrap();

        // Add to the total disk usage.
        total_usage += percent_usage;

        // Print the info the user wants.
        println!("{0: <7} | {1: <43} |{2: <7}", p.proc_id, p.proc_name, p.proc_disk_usage);
    }
    // Output the total disk allocation.
    println!("TOTAL DISK USAGE OVER LAST INTERVAL: {0: <7}", format_percent_usage(total_usage));

    // Send back total_usage for testing purposes.
    return total_usage;
}

pub fn display_net_info() -> f32 {
    let process_info = database::get_current_metrics_from_db();

    // String to be parsed into a float.
    let mut percent_usage_string: String = "".to_owned();

    // Variable to store percents as floats.
    let mut percent_usage: f32 = 0.0;

    // Used to find sum of all usages.
    let mut total_usage: f32 = 0.0;

    println!("{0: <7} | {1: <43} |{2: <7}", "PID", "NAME", "NETWORK");
    println!("---------------------------------------------------------");
    for p in process_info.unwrap() {
        // Get the percent usage string.
        percent_usage_string = p.proc_net_usage.replace("%", "");

        // Convert the percent usage string to a float.
        percent_usage = percent_usage_string.parse::<f32>().unwrap();

        // Add to the total disk usage.
        total_usage += percent_usage;

        // Print the info the user wants.
        println!("{0: <7} | {1: <43} |{2: <7}", p.proc_id, p.proc_name, p.proc_net_usage);
    }
    // Output the total disk allocation.
    println!("TOTAL NETWORK USAGE OVER LAST INTERVAL: {0: <7}", format_percent_usage(total_usage));

    // Send back total_usage for testing purposes.
    return total_usage;
}

pub fn display_help_info() {
    println!("Marist Metrics Collector CLI Commands:");
    println!("     M -> Display all metrics information in the database");
    println!("     CPU -> Display CPU usage of all processes as a percent");
    println!("     Disk -> Display disk usage of all processes as a percent");
    println!("     NET -> Display network usage of all processes as a percent");
    println!("     Exit -> Quit the application");
}

#[cfg(test)]
mod cli_command_tests {
    use crate::cli_commands::{display_cpu_info, display_disk_info, display_net_info};

    #[test]
    fn test_display_cpu_info() {
        assert!(display_cpu_info() <= 100.0);
    }

    #[test]
    fn test_display_disk_info() {
        assert!(display_disk_info() <= 100.0);
    }

    #[test]
    fn test_display_net_info() {
        assert!(display_net_info() <= 100.0);
    }
}