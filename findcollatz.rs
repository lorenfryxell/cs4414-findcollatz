//Returns the Number of Collatz Steps for Value n
fn collatz_steps(n: int) -> int {
    if n == 1 {
        0
    } else {
        1 + collatz_steps(if n % 2 == 0 { n / 2 } else { 3*n + 1 })
    }
}

//Tuple Struct for Sending Data Between Tasks
struct Tuple {
    id: int,
    val: int,
    steps: int
}

//Returns the Minimum Value, with Collatz Stopping Time >= k
fn find_collatz(k: int) -> int {

    //Set the Number of Tasks Spawned
    let tasks = 7;

    //Shared Channel for Sending Tuples From Spawned Task to Host Task
    let (port, chan) = SharedChan::new();

    //Vector of Channels so Host can Tell Spawned Tasks to Finish
    let mut chans = ~[];

    //Start Tasks
    for i in range(0, tasks) {
        
        //Shared Sending Channel
        let chan = chan.clone();

        //Custom (Individual) Receiving Channel
        let (portx, chanx) : (Port<int>, Chan<int>) = Chan::new();
        chans.push(chanx);
        
        //Set First Val to Search For
        let init_val = 1 + i;

        //Set ID of Process
        let id = i;

        //Spawn the Process
        spawn(proc() { 
            let mut val = init_val;
            let mut steps = -1;
            let mut done = false;
            loop {
                //Listen for Host's Message to Finish
                match portx.try_recv() {
                    Some(v) => {    //Finish
                        loop {
                            steps = collatz_steps(val);
                            if steps >= k {
                                println!("Process {:d} was told to finish and found valid result for value: {} steps: {}", id, val, steps);
                                break;
                            }
                            if val > v {
                                println!("Process {:d} was told to finish and stopped at value: {} steps: {}", id, val, steps);
                                break;
                            }
                            val += tasks;            
                        }
                        done = true;
                    }
                    None => {}      //Continue
                }
                if (done) {
                    break;
                }
                steps = collatz_steps(val);
                if steps >= k {
                    println!("Process {:d} finished FIRST (as far as it knows) and found valid result for value: {} steps: {}", id, val, steps);
                    break;
                }
                //Each Task Will Explore a Non-Overlapping Trajectory of Values
                val += tasks;
            }
            //Upon Finishing, Send Tuple Back to Host
            let tup = Tuple { id: id, val: val, steps: steps };
            chan.send(tup);
        });
    }

    //Listen for Result (first task to respond)
    let final_tup = port.recv();

    //Once Result is Found, Tell Other Tasks to Finish Up
    for i in range(0, tasks) {
        if (i != final_tup.id) {
            chans[i].try_send(final_tup.val);    
        }
    }

    //Set (temporary) Final Value
    let mut final_value = final_tup.val;
    let mut final_steps = final_tup.steps;

    //Check Remaining Processes for a Smaller Value with Steps > k, Assuring True Final Value
    for i in range(0, tasks-1) {
        let check_tup = port.recv();
        if check_tup.steps >= k && check_tup.val < final_value {
            final_value = check_tup.val;
            final_steps = check_tup.steps;
        }
    }

    //Return Final Value
    assert!(final_value != -1);
    final_value
}

fn main() {
    println!("Final Result: {}", find_collatz(700));
}