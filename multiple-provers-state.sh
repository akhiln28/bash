#! /bin/bash
function check_port() {
    # echo "Checking the name of process listening on port $1..."
    process_name=$(ssh $2 "lsof -i:$1 | grep LISTEN | awk '{print \$1}'")
    # if process_name is empty, then the process is not running
    if [ -z "$process_name" ]; then
        echo -e "\033[38;5;208mProcess listening on port $1 not found on $2! \033[0m"
        return
    else
        # echo -e "\033[38;5;208m Process $process_name on port $1 found on $2! \033[0m"
        echo -e "\033[38;5;4mProcess $process_name on port $1 found on $2!\033[0m"
    fi
}

function check_all_ports() {
    check_port 3030 "SEQUENCER"
    check_port 8080 "MAIN_PROVER"
    check_port 9090 "PROVER_1"
    check_port 9090 "PROVER_2"
    check_port 9090 "PROVER_3"
    check_port 9090 "PROVER_4"
}

function check_latest_log() {
    # $1 log file prefix
    # $2 remote machine
    log_file_name=$(ssh $2 "ls -t $1_$2* | head -n1")
    echo -e "\033[38;5;208m$2 LOGS... $log_file_name \033[0m"
    ssh $2 "tail -n 500 \$(ls -t $1_$2* | head -n1)"
}

function check_live_log() {
    # $1 log file prefix
    # $2 remote machine
    log_file_name=$(ssh $2 "ls -t $1_$2* | head -n1")
    echo -e "\033[38;5;208m$2 LOGS... $log_file_name \033[0m"
    ssh $2 "tail -n 500 -f \$(ls -t $1_$2* | head -n1)"
}

function save_latest_log() {
    # $1 log file prefix
    # $2 remote machine
    # $3 local machine folder
    log_file_name=$(ssh $2 "ls -t $1_$2* | head -n1")
    echo -e "\033[38;5;208m$2 LOGS... $log_file_name \033[0m"
    scp $2:~/$log_file_name $3/$2
}

function check_all_logs() {
    if [[ $1 == "sequencer" ]]; then
        check_latest_log "sequencer_binary" "SEQUENCER"
    elif [[ $1 == "main_prover" ]]; then
        check_latest_log "main_prover_binary" "MAIN_PROVER"
    elif [[ $1 == "prover_1" ]]; then
        check_latest_log "prover_binary" "PROVER_1"
    elif [[ $1 == "prover_2" ]]; then
        check_latest_log "prover_binary" "PROVER_2"
    elif [[ $1 == "prover_3" ]]; then
        check_latest_log "prover_binary" "PROVER_3"
    elif [[ $1 == "prover_4" ]]; then
        check_latest_log "prover_binary" "PROVER_4"
    else
        check_latest_log "sequencer_binary" "SEQUENCER"
        check_latest_log "main_prover_binary" "MAIN_PROVER"
        check_latest_log "prover_binary" "PROVER_1"
        check_latest_log "prover_binary" "PROVER_2"
        check_latest_log "prover_binary" "PROVER_3"
        check_latest_log "prover_binary" "PROVER_4"
    fi
}

if [[ $1 == "ports" ]]; then
    check_all_ports
elif [[ $1 == "logs" ]]; then
    check_all_logs $2
elif [[ $1 == "live_logs" ]]; then
    if [[ $2 == "sequencer" ]]; then
        check_live_log "sequencer_binary" "SEQUENCER"
    elif [[ $2 == "main_prover" ]]; then
        check_live_log "main_prover_binary" "MAIN_PROVER"
    elif [[ $2 == "prover_1" ]]; then
        check_live_log "prover_binary" "PROVER_1"
    elif [[ $2 == "prover_2" ]]; then
        check_live_log "prover_binary" "PROVER_2"
    elif [[ $2 == "prover_3" ]]; then
        check_live_log "prover_binary" "PROVER_3"
    elif [[ $2 == "prover_4" ]]; then
        check_live_log "prover_binary" "PROVER_4"
    else
        echo "Invalid argument!"
    fi
elif [[ $1 == "save_logs" ]]; then
    save_latest_log "sequencer_binary" "SEQUENCER" $2
    save_latest_log "main_prover_binary" "MAIN_PROVER" $2
    save_latest_log "prover_binary" "PROVER_1" $2
    save_latest_log "prover_binary" "PROVER_2" $2
    save_latest_log "prover_binary" "PROVER_3" $2
    save_latest_log "prover_binary" "PROVER_4" $2
else
    echo "Invalid argument!"
fi

