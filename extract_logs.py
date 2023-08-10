import os
from datetime import datetime, timedelta
import sys

level_1_columns = [
    'Tree update time',
    'Trace generation time',
    'Trace polynomial time',
    'Trace extension time',
    'Trace GMIMC hash time',
    'Individual constraints time',
    'Group and merge time',
    'Composition polynomial time',
    'Composition extension time',
    'Composition GMIMC hash time',
    'OOD Frame time',
    'Deep composition extension time',
    'Trace extension merkle tree time',
    'Composition extension merkle tree time',
    'Final deep composition time',
    'Fri commit time',
    'Fri proof time',
    'Trace extension proof time',
    'Composition extension proof time',
    'Level 1 prover time',
]

level_2_columns = [
    'Trace polynomial time',
    'Trace extension time',
    'Trace commit time',
    'Individual constraints time',
    'Group and merge time',
    'Composition polynomial time',
    'Composition extension time',
    'Composition commit time',
    'OOD Frame time',
    'Deep composition extension time',
    'Fri commit time',
    'Fri proof time',
    'Trace extension proof time',
    'Composition extension proof time',
    'Level 2 prover time'
]

level_1_sizes = [
    'OOD frame size',
    'Fri proof size',
    'Fri roots size',
    'Trace extension proof size',
    'Composition extension proof size',
    'Fri output size',
    'Level 1 proof size',
]

level_2_sizes = [
    'OOD frame size',
    'Fri roots size',
    'Trace extension proof size',
    'Composition extension proof size',
    'Fri proof size',
    'Level 2 proof size',
]


PROVER_1 = """
Setting Prover ID: 1
Server started, listening on 0.0.0.0:9090
[2m2023-08-04T11:27:54.169261Z[0m [32m INFO[0m [2mmain[0m[2m:[0m Listening on 0.0.0.0:9090
Total number of public inputs in the pool are 1
time taken to compute trace polynomials is (layer 1) 4.039202371s
Total number of trace polynomials in the pool are 1
time taken to compute trace extension is (layer 1) 32.619749448s
Total number of trace extension in the pool are 1
GPU is available
Computing gmimc on GPU with 4194304 rows
Creating input buffer of size 1157627904 u64 numbers
Writing input buffer
Time to write buffer: 23026 ms
Kernel execution + time to read buffer: 17610 ms
Time to convert to field elements: 113 ms
time taken gmimc hash (trace extension) is 49.143965715s
Evaluating constraints
time taken for individual constraints is 89.103983673s
Individual Constraint evaluation done
Total number of individual constraints in the pool are 1
Evaluating Compositon polynomial
time taken for group and merge is 94.725612186s
time taken for composition polynomial is 60.357810009s
Total number of trace extension in the pool are 1
time taken for composition extension is 12.797128004s
Total number of trace extension in the pool are 1
Compositon polynomial evaluation done
time taken for composition gmimc hash is 63.475884289s
Evaluating Deep polynomial
time taken to compute ood frame (level 1) is 2.91815067s
time takne for deep composition extension is 16.589828507s
deep composition evaluation done
Fetching trace Extension at query positions
time taken to query trace extension at query positions is 86.39µs
time taken to query composition extension at query positions is 34.481µs
fetching done
"""

SEQUENCER = """
Loaded 128 number of users from db
Server started, listening on 0.0.0.0:3030
[2m2023-08-04T11:27:52.125365Z[0m [32m INFO[0m [2mmain[0m[2m:[0m Listening on 0.0.0.0:3030
Started block generation 1
---------------------------------------------------------
Number of transactions: 2048.
Hold on.. Generating trace.
time taken for tree updates is 613.292977996s
time taken to generate trace is 8.188562454s
server P* response is "true"
---------------------------------------------------------
Number of transactions: 2048.
Hold on.. Generating trace.
time taken for tree updates is 613.103553361s
time taken to generate trace is 8.160711046s
server P* response is "true"
---------------------------------------------------------
Number of transactions: 2048.
Hold on.. Generating trace.
time taken for tree updates is 613.237831907s
time taken to generate trace is 8.161752348s
server P* response is "true"
---------------------------------------------------------
Number of transactions: 2048.
Hold on.. Generating trace.
time taken for tree updates is 613.086156229s
time taken to generate trace is 8.154613618s
server P* response is "true"
Block generated succesfully 1
"""

MAIN_PROVER = """
Server started, listening on 0.0.0.0:8080
[2m2023-08-04T11:27:52.390263Z[0m [32m INFO[0m [2mmain[0m[2m:[0m Listening on 0.0.0.0:8080
Server responded succesfully(request 1)
Public input added into data pool with id 0
Total number of trace hash table in the pool are 1
Waiting to finish other task
Server responded succesfully(request 2)
Waiting to finish other tasks.......
Server responded succesfully(request 1)
Public input added into data pool with id 1
Total number of trace hash table in the pool are 2
Waiting to finish other task
Server responded succesfully(request 2)
Waiting to finish other tasks.......
Server responded succesfully(request 1)
Public input added into data pool with id 2
Total number of trace hash table in the pool are 3
Waiting to finish other task
Server responded succesfully(request 2)
Waiting to finish other tasks.......
Server responded succesfully(request 1)
Public input added into data pool with id 3
Total number of trace hash table in the pool are 4
Finished all task..
Generating merkle tree
time taken to generate layer 1 trace extension merkle tree is 1.996760725s
Merkle tree generation done
Server responded succesfully(request 2)
All task finished.......
Preparing request to layer 1 prover to compute Composition Extension
Sending request to the prover 0 of layer 1
Sending request to the prover 1 of layer 1
Sending request to the prover 3 of layer 1
Sending request to the prover 2 of layer 1
time taken to get response from prover 3 (composition computation) 228.417176322s is
time taken to get response from prover 2 (composition computation) 232.882068244s is
time taken to get response from prover 1 (composition computation) 234.921594527s is
time taken to get response from prover 0 (composition computation) 236.406358068s is
Total number of composition hash table in the pool are 1
Total number of composition hash table in the pool are 2
Total number of composition hash table in the pool are 3
Total number of composition hash table in the pool are 4
Second Merkle tree generation started
time taken to generate layer 1 composition extension merkle tree is 2.029738894s
Second Merkle tree generation done
Preparing request to layer 1 provers to compute Deep composition extension
Sending request to the prover 3 of layer 1
Sending request to the prover 0 of layer 1
Sending request to the prover 1 of layer 1
Sending request to the prover 2 of layer 1
time taken to get response from prover 3 (deep composition computation) 29.323232851s is
time taken to get response from prover 2 (deep composition computation) 31.277615637s is
time taken to get response from prover 1 (deep composition computation) 31.740392493s is
time taken to get response from prover 0 (deep composition computation) 32.089638977s is
Total number of oodframe in the pool are 1
Total number of deep polynomials in the pool are 1
Total number of oodframe in the pool are 2
Total number of deep polynomials in the pool are 2
Total number of oodframe in the pool are 3
Total number of deep polynomials in the pool are 3
Total number of oodframe in the pool are 4
Total number of deep polynomials in the pool are 4
time taken to compute deep composition coeffiecients and ood points is 334.129µs
time taken for layer 1 pow is 95.588082ms
time taken to merge all deep composition is 328.468806ms
computing fri proof of final deep composition poly
time taken to generate layer 1 trace extension proof is 176.436959ms
time taken to generate layer 1 composition extension proof is 173.527974ms
time taken for prover(query trace) 1, to response is 4.293004249s
time taken for prover(query trace) 2, to response is 4.463076649s
time taken for prover(query trace) 0, to response is 4.490668634s
Total number of query_table in the pool are 1
Total number of query_table in the pool are 2
Total number of query_table in the pool are 3
time taken for prover(query trace) 3, to response is 9.838952832s
Total number of query_table in the pool are 4
trace computations started
time taken to compute level 2 trace is 7.65364288s
trace computation done
time taken for layer 1 fri commits is 81.21788773s
time taken for layer 1 fri proof is 794.596µs
Fri proof computed succesfully
Proof generation started
trace length 1 is 2097152
trace length 2 is 524288
time taken for the trace polynomial is 3.820407511s
1 is processing
1 ended 2nd is processing
2nd ended 3rd is processing
3rd ended 4th is processing
4th ended 5th is processing
column 5 is processing
column 5 ended adn column 2 processing
column 2 process ended and boundary table processing
boundary ended
fri ended
time taken for trace extension is 235.259403722s
time taken for trace commit is 49.003426097s
time taken for individual constraints is 10.286312867s
GPU is available
Creating input buffer of size 503316480 u32 numbers
Writing input buffer
Wrote input buffer in 429 ms at the speed of 4.370629370629371 GiB per second
Creating input buffer of size 335544320 u32 numbers
Writing input buffer
Wrote input buffer in 286 ms at the speed of 4.370629370629371 GiB per second
Creating input buffer of size 16777224 u32 numbers
Writing input buffer
Wrote input buffer in 14 ms at the speed of 4.464287843023028 GiB per second
Creating input buffer of size 67108872 u32 numbers
Writing input buffer
Wrote input buffer in 57 ms at the speed of 4.385965435128463 GiB per second
Creating input buffer of size 480 u32 numbers
Writing input buffer
Wrote input buffer in 0 ms at the speed of inf GiB per second
Creating input buffer of size 80 u32 numbers
Writing input buffer
Wrote input buffer in 0 ms at the speed of inf GiB per second
Creating input buffer of size 605 u32 numbers
Writing input buffer
Wrote input buffer in 0 ms at the speed of inf GiB per second
Creating input buffer of size 605 u32 numbers
Writing input buffer
Wrote input buffer in 0 ms at the speed of inf GiB per second
Created output buffer of size 16777216 in 0 ms at the rate of inf GiB per second
Created output buffer of size 67108864 in 1 ms at the rate of 250 GiB per second
Time to write buffer: 936 ms
GPU failed, falling back to CPU
time taken for group and merge is 135.348829905s
time taken for composition polynomial is 15.783395168s
time taken for composition extension is 76.20472981s
time taken for composition commit is 16.257698996s
The time taken for OOD frame is 2.885954545s
time taken for deep composition extension is 60.257616718s
time taken for fri commits is 216.703847564s
time taken to generate fri proof is 393.022µs
time taken for trace extension proof is 173.898µs
The time taken for composition extension proof is 407.472527ms
time taken to generate layer 2 proof is 835.383909398s
------------------------------------------------------
size of layer 2 ood frame is 1.515625
size of layer 2 fri roots is 0.3203125
size of layer 2 trace extension proof is 64.71484375
size of layer 2 Composition extension proof  is 27.33984375
FRI proof size is 89.5576171875
Layer 2 proof size is 183.5224609375
------------------------------------------------------
------------------------------------------------------
size of public inputs is 0.25
size of layer 1 ood frame is 18.828125
size of layer 1 trace extension root is 0.03515625
size of layer 1 composition extension roots is 0.03515625
size of layer 1 fri proof is 155.0791015625
size of layer 1 fri roots is 0.53125
size of layer 1 trace extension proof is 23.2900390625
size of layer 1 Composition extension proof  is 23.2900390625
size of fri output(DC value) is 16.0078125
layer 1 proof size is 237.3583984375
------------------------------------------------------
time taken for the layer 1 ood check is 6.252042ms
time taken to compute dc coeff and dc ood values is 43.033678ms
time taken for layer 1 fri intial check is 44.041787ms
time taken for layer 1 fri test is 23.354449ms
time taken for layer 2 ood check is 17.274083ms
time taken for the layer 2 fri initial check is 10.58078ms
time taken for layer 2 fri proof is 10.510012ms
Proof verified in 155.7 ms
"""

level_1_columns_prefix_in_logs = {
    'Tree update time': ('SEQUENCER', 'time taken for tree updates is '),
    'Trace generation time': ('SEQUENCER', 'time taken to generate trace is '),
    'Trace polynomial time': ('PROVER_1', 'time taken to compute trace polynomials is (layer 1) '),
    'Trace extension time': ('PROVER_1', 'time taken to compute trace extension is (layer 1) '),
    'Trace GMIMC hash time': ('PROVER_1', 'time taken gmimc hash (trace extension) is '),
    'Individual constraints time': ('PROVER_1', 'time taken for individual constraints is '),
    'Group and merge time': ('PROVER_1', 'time taken for group and merge is '),
    'Composition polynomial time': ('PROVER_1', 'time taken for composition polynomial is '),
    'Composition extension time': ('PROVER_1', 'time taken for composition extension is '),
    'Composition GMIMC hash time': ('PROVER_1', 'time taken for composition gmimc hash is '),
    'OOD Frame time': ('PROVER_1', 'time taken to compute ood frame (level 1) is '),
    'Deep composition extension time': ('PROVER_1', 'time takne for deep composition extension is '),
    'Trace extension merkle tree time': ('PROVER_1', 'time taken to query trace extension at query positions is '),
    'Composition extension merkle tree time': ('PROVER_1', 'time taken to query composition extension at query positions is '),
    'Final deep composition time': ('MAIN_PROVER', 'time taken for final_deep_composition is '),
    'Fri commit time': ('MAIN_PROVER', 'time taken for layer 1 fri commits is '),
    'Fri proof time': ('MAIN_PROVER', 'time taken for layer 1 fri proof is '),
    'Trace extension proof time': ('MAIN_PROVER', 'time taken for trace extension proof is '),
    'Composition extension proof time': ('MAIN_PROVER', 'The time taken for composition extension proof is '),
    'Level 1 prover time': None
}

level_2_columns_prefix_in_logs = {
    'Trace polynomial time': ('MAIN_PROVER', 'time taken for the trace polynomial is '),
    'Trace extension time': ('MAIN_PROVER', 'time taken for trace extension is '),
    'Trace commit time': ('MAIN_PROVER', 'time taken for trace commit is '),
    'Individual constraints time': ('MAIN_PROVER', 'time taken for individual constraints is '),
    'Group and merge time': ('MAIN_PROVER', 'time taken for group and merge is '),
    'Composition polynomial time': ('MAIN_PROVER', 'time taken for composition polynomial is '),
    'Composition extension time': ('MAIN_PROVER', 'time taken for composition extension is '),
    'Composition commit time': ('MAIN_PROVER', 'time taken for composition commit is '),
    'OOD Frame time': ('MAIN_PROVER', 'The time taken for OOD frame is '),
    'Deep composition extension time': ('MAIN_PROVER', 'time taken for deep composition extension is '),
    'Fri commit time': ('MAIN_PROVER', 'time taken for fri commits is '),
    'Fri proof time': ('MAIN_PROVER', 'time taken to generate fri proof is '),
    'Trace extension proof time': ('MAIN_PROVER', 'time taken for trace extension proof is '),
    'Composition extension proof time': ('MAIN_PROVER', 'The time taken for composition extension proof is '),
    'Level 2 prover time': ('MAIN_PROVER', 'time taken to generate layer 2 proof is '),
}

level_1_sizes_prefix_in_logs = {
    'OOD frame size': ('MAIN_PROVER', 'size of layer 1 ood frame is '),
    'Fri proof size': ('MAIN_PROVER', 'size of layer 1 fri proof is '),
    'Fri roots size': ('MAIN_PROVER', 'size of layer 1 fri roots is '),
    'Trace extension proof size': ('MAIN_PROVER', 'size of layer 1 trace extension proof is '),
    'Composition extension proof size': ('MAIN_PROVER', 'size of layer 1 Composition extension proof  is '),
    'Fri output size': ('MAIN_PROVER', 'size of fri output(DC value) is '),
    'Level 1 proof size': ('MAIN_PROVER', 'layer 1 proof size is '),
}

level_2_sizes_prefix_in_logs = {
    'OOD frame size': ('MAIN_PROVER', 'size of layer 2 ood frame is '),
    'Fri roots size': ('MAIN_PROVER', 'size of layer 2 fri roots is '),
    'Trace extension proof size': ('MAIN_PROVER', 'size of layer 2 trace extension proof is '),
    'Composition extension proof size': ('MAIN_PROVER', 'size of layer 2 Composition extension proof  is '),
    'Fri proof size': ('MAIN_PROVER', 'FRI proof size is '),
    'Level 2 proof size': ('MAIN_PROVER', 'Layer 2 proof size is '),
}


def extract_from_prefix(log_file_str: str, prefix: str) -> str:
    lines = log_file_str.split('\n')
    for line in lines:
        if line.startswith(prefix):
            return line[len(prefix):]
    return None


def extract_time_duration_from_str(time_str: str) -> timedelta:
    time_str = time_str.strip()
    if time_str.endswith('µs'):
        return timedelta(microseconds=float(time_str[:-2]))
    elif time_str.endswith('ms'):
        return timedelta(milliseconds=float(time_str[:-2]))
    elif time_str.endswith('s'):
        return timedelta(seconds=float(time_str[:-1]))
    else:
        return timedelta()


def extract_level1_cols(dr: dict):
    ans: dict = {}
    for key, value in dr.items():
        if key == 'Level 1 prover time':
            continue
        log_file = os.path.join(logs_dir, value[0])
        log_file_str = open(log_file).read()
        prefix = value[1]
        val = extract_from_prefix(log_file_str, prefix) or ""
        ans[key] = extract_time_duration_from_str(val).total_seconds()
    total_time = 0
    for key in [
        'Trace polynomial time',
        'Trace extension time',
        'Trace GMIMC hash time',
        'Individual constraints time',
        'Group and merge time',
        'Composition polynomial time',
        'Composition extension time',
        'Composition GMIMC hash time',
        'OOD Frame time',
        'Deep composition extension time',
        'Trace extension merkle tree time',
        'Composition extension merkle tree time',
        'Final deep composition time',
        'Fri commit time',
        'Fri proof time',
        'Trace extension proof time',
        'Composition extension proof time',
    ]:
        total_time += ans[key]
    ans['Level 1 prover time'] = total_time
    print(','.join([str(v) for v in ans.values()]))


def extract_time_cols(dr: dict):
    ans: dict = {}
    for key, value in dr.items():
        log_file = os.path.join(logs_dir, value[0])
        log_file_str = open(log_file).read()
        prefix = value[1]
        val = extract_from_prefix(log_file_str, prefix) or ""
        ans[key] = extract_time_duration_from_str(val).total_seconds()
    # print(','.join(ans.values()))
    print(','.join([str(v) for v in ans.values()]))

def extract_cols(dr: dict):
    ans: dict = {}
    for key, value in dr.items():
        log_file = os.path.join(logs_dir, value[0])
        log_file_str = open(log_file).read()
        prefix = value[1]
        ans[key] = extract_from_prefix(log_file_str, prefix) or ""
    print(','.join(ans.values()))


current_dir = os.path.dirname(os.path.abspath(__file__))
folder = sys.argv[1]
logs_dir = os.path.join(current_dir, folder)
MAIN_PROVER_file = os.path.join(logs_dir, 'MAIN_PROVER')
SEQUENCER_file = os.path.join(logs_dir, 'SEQUENCER')
PROVER_1_file = os.path.join(logs_dir, 'PROVER_1')

print('number of transactions =', folder)
print('Level 1 columns')
extract_level1_cols(level_1_columns_prefix_in_logs)

print('\nLevel 2 columns')
extract_time_cols(level_2_columns_prefix_in_logs)

print('\nLevel 1 sizes')
extract_cols(level_1_sizes_prefix_in_logs)

print('\nLevel 2 sizes')
extract_cols(level_2_sizes_prefix_in_logs)

# Proof verified in 84.4 ms
verification_time = extract_from_prefix(open(MAIN_PROVER_file).read(), 'Proof verified in ') or ""
print('\nVerification time =', extract_time_duration_from_str(verification_time).total_seconds())