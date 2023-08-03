#! /bin/bash
function kill_process() {
    # if the process is not running, don't do anything
    if [ -z "$(ssh $2 "pgrep -f ./$1")" ]; then
        echo "Process $1 on $2 not running!"
        return
    fi
    echo "Killing $1 process on $2..."
    ssh $2 "pkill -f ./$1"
    if [ $? -eq 0 ]; then
        echo "Existing process $1 on $2 killed successfully!"
    else
        echo "Existing process $1 on $2 kill failed with error code $?"
        exit 1
    fi
}

kill_process "main_prover_binary" "MAIN_PROVER"
kill_process "sequencer_binary" "SEQUENCER"
kill_process "prover_binary" "PROVER_1"
kill_process "prover_binary" "PROVER_2"
kill_process "prover_binary" "PROVER_3"
kill_process "prover_binary" "PROVER_4"

# build sequencer, main prover and prover
echo "Building sequencer binary..."
ssh MAIN_PROVER "cd ~/test_sequencer; ~/.cargo/bin/cargo build --release; cp target/release/main ~/sequencer_binary;"
if [ $? -eq 0 ]; then
    echo "Sequencer binary built successfully!"
else
    echo "Sequencer binary build failed with error code $?"
    exit 1
fi

echo "Building main prover binary..."
ssh MAIN_PROVER "cd ~/main_prover; ~/.cargo/bin/cargo build --release; cp target/release/main ~/main_prover_binary;"
if [ $? -eq 0 ]; then
    echo "Main prover binary built successfully!"
else
    echo "Main prover binary build failed with error code $?"
    exit 1
fi

echo "Building prover binary..."
ssh MAIN_PROVER "cd ~/prover; ~/.cargo/bin/cargo build --release; cp target/release/main ~/prover_binary;"
if [ $? -eq 0 ]; then
    echo "Prover binary built successfully!"
else
    echo "Prover binary build failed with error code $?"
    exit 1
fi

echo "All binaries built successfully!"

function copy_binary() {
    echo "Copying $1 from MAIN_PROVER to $2..."
    scp MAIN_PROVER:~/$1 $2:~/$1
    if [ $? -eq 0 ]; then
        echo "$1 copied successfully!"
    else
        echo "$1 copy failed with error code $?"
        exit 1
    fi
}
copy_binary "sequencer_binary" "SEQUENCER"
copy_binary "prover_binary" "PROVER_1"
copy_binary "prover_binary" "PROVER_2"
copy_binary "prover_binary" "PROVER_3"
copy_binary "prover_binary" "PROVER_4"

# run binaries on MAIN_PROVER, SEQUENCER, PROVER_1, PROVER_2, PROVER_3, PROVER_4
function run_binary() {
    echo "Running $1 on $2..."
    # $3 contains the prover id, if it is not set, then it is the sequencer or main prover
    ssh $2 "cd ~/; nohup ./$1 $3 > $1_$2_log_$(date +%Y-%m-%d_%H-%M-%S).log 2>&1 & disown";
    if [ $? -eq 0 ]; then
        echo "$1 binary running successfully on $2!"
    else
        echo "$1 binary run failed with error code $? on $2!"
    fi
}
run_binary "sequencer_binary" "SEQUENCER"
run_binary "main_prover_binary" "MAIN_PROVER"
run_binary "prover_binary" "PROVER_1" "0"
run_binary "prover_binary" "PROVER_2" "1"
run_binary "prover_binary" "PROVER_3" "2"
run_binary "prover_binary" "PROVER_4" "3"
echo "All binaries running successfully!"

echo "Sending generate_trace request to SEQUENCER..."
ssh SEQUENCER "curl http://localhost:3030/block/generate_trace"
echo "generate_trace request sent successfully!"
