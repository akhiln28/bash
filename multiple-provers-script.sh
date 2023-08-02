# use bash shell
#! /bin/bash
echo "Killing existing processes on MAIN_PROVER..."
function kill_process() {
    echo "Killing $1 process on $2..."
    ssh $2 "pkill -f ./$1"
    if [ $? -eq 0 ]; then
        echo "Existing process $1 on $2 killed successfully!"
    else
        echo "Existing process $1 on $2 kill failed with error code $?"
    fi
}
# ssh MAIN_PROVER "pkill -f ./main_prover_binary"
# if [ $? -eq 0 ]; then
#     echo "Existing processes on MAIN_PROVER killed successfully!"
# else
#     echo "Existing processes on MAIN_PROVER kill failed with error code $?"
# fi
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
fi

echo "Building main prover binary..."
ssh MAIN_PROVER "cd ~/main_prover; ~/.cargo/bin/cargo build --release; cp target/release/main ~/main_prover_binary;"
if [ $? -eq 0 ]; then
    echo "Main prover binary built successfully!"
else
    echo "Main prover binary build failed with error code $?"
fi

echo "Building prover binary..."
ssh MAIN_PROVER "cd ~/prover; ~/.cargo/bin/cargo build --release; cp target/release/main ~/prover_binary;"
if [ $? -eq 0 ]; then
    echo "Prover binary built successfully!"
else
    echo "Prover binary build failed with error code $?"
fi

echo "All binaries built successfully!"

# transfer binaries from MAIN_PROVER to SEQUENCER, PROVER_1, PROVER_2, PROVER_3, PROVER_4
# scp MAIN_PROVER:~/sequencer_binary SEQUENCER:~/sequencer_binary
# if [ $? -eq 0 ]; then
#     echo "sequencer_binary copied successfully!"
# else
#     echo "sequencer_binary copy failed with error code $?"
# fi
function copy_binary() {
    echo "Copying $1 from MAIN_PROVER to $2..."
    scp MAIN_PROVER:~/$1 $2:~/$1
    if [ $? -eq 0 ]; then
        echo "$1 copied successfully!"
    else
        echo "$1 copy failed with error code $?"
    fi
}
copy_binary "sequencer_binary" "SEQUENCER"
# copy_binary "main_prover_binary" "MAIN_PROVER"
copy_binary "prover_binary" "PROVER_1"
copy_binary "prover_binary" "PROVER_2"
copy_binary "prover_binary" "PROVER_3"
copy_binary "prover_binary" "PROVER_4"

# run binaries on MAIN_PROVER, SEQUENCER, PROVER_1, PROVER_2, PROVER_3, PROVER_4
function run_binary() {
    echo "Running $1 binary on $2..."
    ssh $2 "cd ~/; nohup ./$1 > $1_log_$(date +%Y-%m-%d_%H-%M-%S).log 2>&1 & disown";
    if [ $? -eq 0 ]; then
        echo "$1 binary running successfully on $2!"
    else
        echo "$1 binary run failed with error code $? on $2!"
    fi
}
run_binary "sequencer_binary" "SEQUENCER"
run_binary "main_prover_binary" "MAIN_PROVER"
run_binary "prover_binary" "PROVER_1"
run_binary "prover_binary" "PROVER_2"
run_binary "prover_binary" "PROVER_3"
run_binary "prover_binary" "PROVER_4"
# echo "Running sequencer binary..."
# ssh SEQUENCER "cd ~/; nohup ./sequencer_binary > sequencer_log_$(date +%Y-%m-%d_%H-%M-%S).log 2>&1 & disown";
# echo "Sequencer binary running successfully!"
echo "All binaries running successfully!"