#!/usr/bin/env bash
set -euo pipefail

# Comprehensive benchmark across methods and lengths.
# Outputs CSV: length,method_id,method_name,elapsed_ns
# Requires the release binary first (will attempt to build if missing).

BIN=./target/release/fastest-fibo
if [ ! -x "$BIN" ]; then
  echo "Building release binary..." >&2
  cargo build --release >&2
fi

# Define lengths. Very large exact Fibonacci (>=1e7) gets huge; we cap for exact methods.
LENGTHS_SMALL=(1 2 5 10 20 50 100 200 500 1000 2000 5000 10000 20000 50000 100000 1000000 10000000 100000000 1000000000)
# For extremely large n (>=1e7) we will only run a reduced set of methods (approx + fast doubling variants) to keep time feasible.

# Method names map (id -> name)
method_name() {
  case "$1" in
    0) echo "matrix_bigint";;
    1) echo "iterative";;
    2) echo "recursive";;
    3) echo "recursive_memo";;
    4) echo "matrix_u128";;
    5) echo "rayon_matrix1";;
    6) echo "rayon_matrix2";;
    7) echo "rayon_matrix3";;
    8) echo "rayon_matrix4";;
    9) echo "threads_matrix1";;
    10) echo "approx1";;
    11) echo "fast_doubling_rec";;
    12) echo "fast_doubling_iter";;
    13) echo "rayon_fast_doubling";;
    14) echo "threads_fast_doubling";;
    *) echo "unknown";;
  esac
}

OUT_DIR=res
mkdir -p "$OUT_DIR"
OUT_CSV="$OUT_DIR/benchmark_all.csv"
echo "length,method_id,method_name,elapsed_ns" > "$OUT_CSV"

# Warmup iterations (kept small to avoid huge total time).
WARMUP=5

for n in "${LENGTHS_SMALL[@]}"; do
  for m in $(seq 0 14); do
    # Reduced set for huge n
    if [ "$n" -ge 10000000 ]; then
      case "$m" in
        10|11|12) ;; # allowed: approximation, fast doubling recursive & iterative
        # Optionally keep rayon fast doubling (13) for comparison at 1e7 only (skip >=1e8)
        13) if [ "$n" -ge 100000000 ]; then continue; fi ;;
        *) continue;;
      esac
    fi
    # Skip obviously intractable combinations:
    # Pure recursive (2) explodes ~phi^n so restrict to n<=40
    if [ "$m" -eq 2 ] && [ "$n" -gt 40 ]; then continue; fi
    # Recursive memo (3) is O(n) but BigInt growth still; allow up to 100000 maybe, ok.
    if [ "$m" -eq 3 ] && [ "$n" -gt 100000 ]; then continue; fi
    # Threads matrix (9) currently WIP; allow but skip if n too small (thread overhead) - optional skip
    if [ "$m" -eq 9 ] && [ "$n" -lt 1000 ]; then continue; fi
    # Matrix bigint (0) and iterative (1) O(n) become very large; skip after 1e6
    if [ "$n" -gt 1000000 ] && { [ "$m" -eq 0 ] || [ "$m" -eq 1 ]; }; then continue; fi
    # Rayon / thread matrix variants become huge; skip after 1e6
    if [ "$n" -gt 1000000 ] && { [ "$m" -ge 5 ] && [ "$m" -le 9 ]; }; then continue; fi
    # u128 matrix (4) overflows early; still include for curiosity up to 1e6, skip beyond
    if [ "$n" -gt 1000000 ] && [ "$m" -eq 4 ]; then continue; fi
    # Approximation (10) fine for all lengths, but we keep same set.

    # Run binary in quiet+no_time false so we can parse timing; we capture stdout.
    # The binary prints: (maybe result) + '\tTime elapsed: 123ns'. We'll grep for number.
  RAW=$("$BIN" -m "$m" --length "$n" --quiet --warmup "$WARMUP" 2>/dev/null || true)
    # Extract the time line using grep.
    TIME_LINE=$(echo "$RAW" | grep -F "Time elapsed:" || true)
    if [ -z "$TIME_LINE" ]; then
      # Re-run with printing time if quiet suppressed it? (quiet only suppresses result). Already prints time.
      # If still empty record NA.
      ELAPSED_NS=NA
    else
      # Use awk to extract the numeric portion (Rust debug format may be like 123ns or 1.23ms)
      # Convert to nanoseconds.
  VALUE=$(echo "$TIME_LINE" | tr 'µ' 'u' | awk '{print $3}')
      # Normalize suffix
      if [[ $VALUE == *ns ]]; then
        NUM=${VALUE%ns}
        ELAPSED_NS=$NUM
      elif [[ $VALUE == *us ]]; then
        NUM=${VALUE%us}
        ELAPSED_NS=$(awk -v v="$NUM" 'BEGIN{printf "%d", v*1000}')
      elif [[ $VALUE == *ms ]]; then
        NUM=${VALUE%ms}
        ELAPSED_NS=$(awk -v v="$NUM" 'BEGIN{printf "%d", v*1000000}')
      elif [[ $VALUE == *s ]]; then
        NUM=${VALUE%s}
        ELAPSED_NS=$(awk -v v="$NUM" 'BEGIN{printf "%d", v*1000000000}')
      else
        ELAPSED_NS=NA
      fi
    fi
    echo "$n,$m,$(method_name $m),$ELAPSED_NS" | tee -a "$OUT_CSV" >/dev/null
  done
  echo "Completed length $n" >&2
done

echo "Benchmark complete. Results in $OUT_CSV" >&2
