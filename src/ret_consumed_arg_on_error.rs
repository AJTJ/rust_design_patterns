use std::sync::mpsc::SendError;

pub fn send(value: String) -> Result<(), SendError<String>> {
    println!("using {value} in a meaningful way");

    use std::time::SystemTime;
    let period = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    if period.subsec_nanos() % 2 == 1 {
        Ok(())
    } else {
        Err(SendError(value))
    }
}

pub fn returned_error() {
    let mut value = "imagine this is very long string".to_string();

    let success = 's: {
        // Try to send value two times.
        for _ in 0..1 {
            value = match send(value) {
                Ok(()) => break 's true,
                Err(SendError(value)) => value,
            }
        }
        false
    };

    // NOTE: How I might normally do it
    // let success = false;
    // for _ in 0..2 {
    //     value = match send(value) {
    //         Ok(()) => {
    //             success = true;
    //             break;
    //         }
    //         Err(SendError(value)) => value,
    //     }
    // }

    println!("success: {}", success);
}
